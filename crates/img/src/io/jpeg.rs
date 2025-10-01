use std::io;

use zune_jpeg::{zune_core::{colorspace::ColorSpace, options::DecoderOptions}, JpegDecoder};

use crate::{error::IoResult, image::Image, pixel::Pixel, prelude::Size};

pub trait ReadJpeg
where
    Self: Sized,
{
    fn read_jpeg(read: impl io::Read) -> IoResult<Self>;
}

impl ReadJpeg for Image {
    fn read_jpeg(mut read: impl io::Read) -> IoResult<Self> {
        let mut data = Vec::new();
        let _ = read.read_to_end(&mut data)?;
        let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
        let mut decoder = JpegDecoder::new_with_options(&data, options);
        let bytes = decoder.decode()?;
        let image_info = decoder.info().unwrap();

        let width: usize = image_info.width.into();
        let height: usize = image_info.height.into();

        let mut pixels = vec![Pixel::zero(); width * height].into_boxed_slice();

        for (target_px, source_px) in
            pixels.iter_mut().zip(bytes.chunks(3))
        {
            target_px.set_r(source_px[0]);
            target_px.set_g(source_px[1]);
            target_px.set_b(source_px[2]);
            target_px.set_a(255);
        }

        let width = width.try_into().expect("invalid width");
        let height = height.try_into().expect("invalid height");

        Ok(Image::new(Size::new(width, height), pixels).unwrap())
    }
}
