use std::io;

use thiserror::Error;

use crate::{
    error::{
        IoError,
        IoResult,
    },
    image::Image,
    pixel::{
        PIXEL_SIZE,
        Pixel,
    },
    prelude::Size,
};

pub trait ReadJpeg
where
    Self: Sized,
{
    fn read_jpeg(read: impl io::Read) -> IoResult<Self>;
}

impl ReadJpeg for Image {
    fn read_jpeg(mut read: impl io::Read) -> IoResult<Self> {
        let mut jpeg_data = Vec::new();
        read.read_to_end(&mut jpeg_data)?;
        let turbojpeg_image = turbojpeg::decompress(&jpeg_data, turbojpeg::PixelFormat::RGBA)
            .map_err(IoError::JpegDecoding)?;

        let size =
            Size::from_usize(turbojpeg_image.width, turbojpeg_image.height).map_err(|_| {
                IoError::Unsupported(
                    "Images with width or height zero are not supported".to_string(),
                )
            })?;
        let image = Image::new(
            size,
            turbojpeg_image
                .pixels
                .chunks(PIXEL_SIZE)
                .map(|c| Pixel::new(c.try_into().unwrap()))
                .collect(),
        )
        .map_err(|_| IoError::Unexpected("Unexpected value received from turbojpeg".to_string()))?;

        Ok(image)
    }
}

#[derive(Debug)]
pub struct JpegQuality(i32);

#[derive(Debug, Error)]
pub enum JpegQualityCreationError {
    #[error("Invalid jpeg quality value")]
    InvalidValue,
}

pub type JpegQualityCreationResult<T> = std::result::Result<T, JpegQualityCreationError>;

impl JpegQuality {
    pub fn new(value: i32) -> JpegQualityCreationResult<Self> {
        if !(0..=100).contains(&value) {
            return Err(JpegQualityCreationError::InvalidValue);
        }

        Ok(Self(value))
    }
}

impl Default for JpegQuality {
    fn default() -> Self {
        Self(75)
    }
}

#[derive(Debug, Default)]
pub enum JpegSubsampling {
    None,
    Sub2x1,
    #[default]
    Sub2x2,
    Sub1x2,
    Sub4x1,
    Sub1x4,
}

impl From<JpegSubsampling> for turbojpeg::Subsamp {
    fn from(value: JpegSubsampling) -> Self {
        match value {
            JpegSubsampling::None => turbojpeg::Subsamp::None,
            JpegSubsampling::Sub2x1 => turbojpeg::Subsamp::Sub2x1,
            JpegSubsampling::Sub2x2 => turbojpeg::Subsamp::Sub2x2,
            JpegSubsampling::Sub1x2 => turbojpeg::Subsamp::Sub1x2,
            JpegSubsampling::Sub4x1 => turbojpeg::Subsamp::Sub4x1,
            JpegSubsampling::Sub1x4 => turbojpeg::Subsamp::Sub1x4,
        }
    }
}

pub trait WriteJpeg {
    fn write_jpeg(
        &self,
        write: impl io::Write,
        quality: JpegQuality,
        subsampling: JpegSubsampling,
    ) -> IoResult<()>;
}

impl WriteJpeg for Image {
    fn write_jpeg(
        &self,
        mut write: impl io::Write,
        quality: JpegQuality,
        subsampling: JpegSubsampling,
    ) -> IoResult<()> {
        let buffer = self.buffer();
        let turbojpeg_image = turbojpeg::Image::<&[u8]> {
            pixels: buffer.as_ref(),
            width: self.size().width().get(),
            pitch: self.size().width().get() * PIXEL_SIZE,
            height: self.size().height().get(),
            format: turbojpeg::PixelFormat::RGBA,
        };

        let buf = turbojpeg::compress(turbojpeg_image, quality.0, subsampling.into())
            .map_err(IoError::JpegEncoding)?;
        write.write_all(buf.as_ref())?;
        Ok(())
    }
}
