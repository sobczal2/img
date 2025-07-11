use png::{BitDepth, ColorType};

use crate::{
    error::{IoError, IoResult},
    image::Image,
    pixel::{PixelMut, PIXEL_SIZE},
};

fn pixel_size_by_color_type(color_type: ColorType) -> usize {
    match color_type {
        ColorType::Grayscale => 1,
        ColorType::Rgb => 3,
        ColorType::Indexed => 1,
        ColorType::GrayscaleAlpha => 2,
        ColorType::Rgba => 4,
    }
}

fn get_red(source: &[u8], color_type: ColorType) -> u8 {
    match color_type {
        ColorType::Grayscale => source[0],
        ColorType::Rgb => source[0],
        ColorType::Indexed => source[0],
        ColorType::GrayscaleAlpha => source[0],
        ColorType::Rgba => source[0],
    }
}

fn get_green(source: &[u8], color_type: ColorType) -> u8 {
    match color_type {
        ColorType::Grayscale => source[0],
        ColorType::Rgb => source[1],
        ColorType::Indexed => source[0],
        ColorType::GrayscaleAlpha => source[0],
        ColorType::Rgba => source[1],
    }
}

fn get_blue(source: &[u8], color_type: ColorType) -> u8 {
    match color_type {
        ColorType::Grayscale => source[0],
        ColorType::Rgb => source[2],
        ColorType::Indexed => source[0],
        ColorType::GrayscaleAlpha => source[0],
        ColorType::Rgba => source[2],
    }
}

fn get_alpha(source: &[u8], color_type: ColorType) -> u8 {
    match color_type {
        ColorType::Grayscale => 255,
        ColorType::Rgb => 255,
        ColorType::Indexed => 255,
        ColorType::GrayscaleAlpha => source[1],
        ColorType::Rgba => source[3],
    }
}

/// Trait for reading png image used in Image struct
pub trait ReadPng
where
    Self: Sized,
{
    fn read_png(read: impl std::io::Read) -> IoResult<Self>;
}

impl ReadPng for Image {
    fn read_png(read: impl std::io::Read) -> IoResult<Self> {
        let decoder = png::Decoder::new(read);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut buf)?;

        if info.bit_depth != BitDepth::Eight {
            return Err(IoError::Unsupported(
                "bit depth different than 8".to_owned(),
            ));
        }

        if info.color_type == ColorType::Indexed {
            return Err(IoError::Unsupported(
                "indexed color type unsupported".to_owned(),
            ));
        }

        let bytes = &buf[..info.buffer_size()];

        let mut image_buf =
            vec![0; info.width as usize * info.height as usize * PIXEL_SIZE].into_boxed_slice();

        for (target_px, source_px) in image_buf
            .chunks_mut(PIXEL_SIZE)
            .zip(bytes.chunks(pixel_size_by_color_type(info.color_type)))
        {
            let mut target = PixelMut::new(target_px.try_into().unwrap());
            target.set_r(get_red(source_px, info.color_type));
            target.set_g(get_green(source_px, info.color_type));
            target.set_b(get_blue(source_px, info.color_type));
            target.set_a(get_alpha(source_px, info.color_type));
        }

        Ok(Image::new((info.width as usize, info.height as usize), image_buf).unwrap())
    }
}

/// Trait for writing png image used in Image struct
pub trait WritePng {
    fn write_png(&self, write: impl std::io::Write) -> Result<(), IoError>;
}

impl WritePng for Image {
    fn write_png(&self, write: impl std::io::Write) -> Result<(), IoError> {
        let mut encoder = png::Encoder::new(write, self.size().0 as u32, self.size().1 as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(self.buf()).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::image::Image;

    #[test]
    fn read_png_success() {
        let data = include_bytes!("../../../assets/sunflower.png");
        Image::read_png(&data[..]).unwrap();
    }

    #[test]
    fn write_png_success() {
        let data = Vec::new();
        Image::empty((10, 10)).write_png(data).unwrap();
    }

    #[test]
    fn write_read_same_image() {
        let mut image = Image::empty((2, 2));
        image.pixel_mut((0, 0)).unwrap().set_r(1);
        image.pixel_mut((0, 1)).unwrap().set_r(1);
        image.pixel_mut((1, 0)).unwrap().set_r(1);
        image.pixel_mut((1, 1)).unwrap().set_r(1);

        let mut data = Vec::new();
        image.write_png(&mut data).unwrap();

        let image2 = Image::read_png(&data[..]).unwrap();

        assert_eq!(image.pixel((0, 0)).unwrap(), image2.pixel((0, 0)).unwrap());
        assert_eq!(image.pixel((0, 1)).unwrap(), image2.pixel((0, 1)).unwrap());
        assert_eq!(image.pixel((1, 0)).unwrap(), image2.pixel((1, 0)).unwrap());
        assert_eq!(image.pixel((1, 1)).unwrap(), image2.pixel((1, 1)).unwrap());
    }
}
