#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use crate::{
    image::Image,
    lens::{FromLens, Lens},
    pixel::{ChannelFlags, Pixel},
};

/// Create a [`Lens`] that applies brightness adjustment to pixels.
///
/// Brightness adjustment multiplies each channel value by a factor:
/// - `factor > 1.0`: Increases brightness (values closer to white)
/// - `factor < 1.0`: Decreases brightness (values closer to black)
/// - `factor == 1.0`: No change
///
/// The resulting values are clamped to the valid range (0-255).
pub fn brightness_lens<S>(source: S, factor: f32, flags: ChannelFlags) -> impl Lens<Item = Pixel>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    source.map(move |px| map_px(px, factor, flags))
}

/// Apply brightness adjustment to an image.
///
/// Brightness adjustment multiplies each channel value by a factor:
/// - `factor > 1.0`: Increases brightness (values closer to white)
/// - `factor < 1.0`: Decreases brightness (values closer to black)
/// - `factor == 1.0`: No change
///
/// The resulting values are clamped to the valid range (0-255).
pub fn brightness(image: &Image, factor: f32, flags: ChannelFlags) -> Image {
    let lens = brightness_lens(image.lens(), factor, flags);
    Image::from_lens(lens)
}

/// Apply brightness adjustment to an image in parallel.
///
/// This is the parallel version of [`brightness`], using multiple threads
/// for processing. Use this for large images where parallel processing
/// provides performance benefits.
#[cfg(feature = "parallel")]
pub fn brightness_par(
    image: &Image,
    threads: NonZeroUsize,
    factor: f32,
    flags: ChannelFlags,
) -> Image {
    use crate::lens::FromLensPar;

    let lens = brightness_lens(image.lens(), factor, flags);
    Image::from_lens_par(lens, threads)
}

/// Internal function to apply brightness adjustment to a single pixel.
fn map_px(px: impl AsRef<Pixel>, factor: f32, flags: ChannelFlags) -> Pixel {
    let px = px.as_ref();

    let new_red = (px.r() as f32 * factor).clamp(0f32, 255f32) as u8;
    let new_green = (px.g() as f32 * factor).clamp(0f32, 255f32) as u8;
    let new_blue = (px.b() as f32 * factor).clamp(0f32, 255f32) as u8;
    let new_alpha = (px.a() as f32 * factor).clamp(0f32, 255f32) as u8;

    let mut px = *px;
    px.set_with_flags(new_red, new_green, new_blue, new_alpha, flags);

    px
}
