use crate::{
    image::Image,
    pipe::{FromPipe, Pipe},
    pixel::Pixel,
};

/// Transform image to grayscale in place
pub fn grayscale(image: &Image) -> Image {
    let view = image.pipe().map(map_px);
    Image::from_pipe(view)
}

fn map_px(px: &Pixel) -> Pixel {
    let value = 0.299 * px.r() as f32 + 0.587 * px.g() as f32 + 0.214 * px.b() as f32;
    let value = value as u8;

    Pixel::new([value, value, value, px.a()])
}
