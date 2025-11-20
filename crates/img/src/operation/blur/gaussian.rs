#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use thiserror::Error;

use crate::{
    component::{
        kernel::gaussian::{GaussianKernel, GaussianKernelCreationError},
        primitive::{
            MarginCreationError,
        },
    },
    image::Image,
    lens::{
        kernel::KernelLensCreationError, FromLens, Lens
    },
    pixel::{
        ChannelFlags,
        Pixel,
    }, prelude::Margin,
};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum GaussianBlurCreationError {
    #[error("failed to create gaussian kernel: {0}")]
    Kernel(#[from] GaussianKernelCreationError),
    #[error("failed to create kernel lens: {0}")]
    KernelLens(#[from] KernelLensCreationError),
    #[error("failed to create margin: {0}")]
    Size(#[from] MarginCreationError),
}

pub type GaussianBlurCreationResult<T> = std::result::Result<T, GaussianBlurCreationError>;

pub fn gaussian_blur_lens<S>(
    source: S,
    radius: usize,
    sigma: f32,
    flags: ChannelFlags,
) -> GaussianBlurCreationResult<impl Lens<Item = Pixel>>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    let kernel = GaussianKernel::new(Margin::unified(radius)?, sigma, flags)?;
    let lens = source.kernel(kernel)?;

    Ok(lens)
}

pub fn gaussian_blur(
    image: &Image,
    radius: usize,
    sigma: f32,
    flags: ChannelFlags,
) -> GaussianBlurCreationResult<Image> {
    let lens = gaussian_blur_lens(image.lens(), radius, sigma, flags)?;
    Ok(Image::from_lens(lens))
}

#[cfg(feature = "parallel")]
pub fn gaussian_blur_par(
    image: &Image,
    threads: NonZeroUsize,
    radius: usize,
    sigma: f32,
    flags: ChannelFlags,
) -> GaussianBlurCreationResult<Image> {
    use crate::lens::FromLensPar;

    let lens = gaussian_blur_lens(image.lens(), radius, sigma, flags)?;
    Ok(Image::from_lens_par(lens, threads))
}
