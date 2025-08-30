#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    image::Image,
    pipe::{FromPipe, IntoPipe, Pipe},
    pixel::Pixel,
};

/// Transform image to grayscale in place
pub fn grayscale(image: Image) -> Image {
    let view = image.into_pipe().map(map_px);
    Image::from_pipe(view)
}

#[cfg(feature = "parallel")]
pub fn grayscale_par(image: Image) {
    image.pixels_mut().par_bridge().for_each(px_to_grayscale);
}

fn map_px<'a>(px: &Pixel) -> Pixel {
    let value = 0.299 * px.r() as f32 + 0.587 * px.g() as f32 + 0.214 * px.b() as f32;
    let value = value as u8;

    Pixel::new([value, value, value, px.a()])
}
