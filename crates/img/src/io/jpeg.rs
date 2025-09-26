use std::io;

use zune_jpeg::{zune_core::{colorspace::ColorSpace, options::DecoderOptions}, JpegDecoder};

use crate::{error::IoResult, image::Image};

pub trait ReadJpeg
where
    Self: Sized,
{
    fn read_png(read: impl io::Read) -> IoResult<Self>;
}

impl ReadJpeg for Image {
    fn read_png(mut read: impl io::Read) -> IoResult<Self> {
        let mut data = Vec::new();
        let _ = read.read_to_end(&mut data)?;
        let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
        let mut decoder = JpegDecoder::new_with_options(&data, options);
        decoder.decode()?;
        let image_info = decoder.info().unwrap();

        todo!()
    }
}
