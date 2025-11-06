#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use thiserror::Error;

use crate::{
    component::{
        kernel::{
            self,
            gaussian::GaussianKernel,
        },
        primitive::{
            Size,
            SizeCreationError,
        },
    },
    image::Image,
    lens::{
        self,
        FromLens,
        Lens,
    },
    pixel::{
        ChannelFlags,
        Pixel,
    },
};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum GaussianBlurCreationError {
    #[error("failed to create gaussian kernel: {0}")]
    Kernel(#[from] kernel::gaussian::CreationError),
    #[error("failed to create kernel lens: {0}")]
    KernelLens(#[from] lens::kernel::CreationError),
    #[error("failed to create size: {0}")]
    Size(#[from] SizeCreationError),
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
    let kernel = GaussianKernel::new(Size::from_radius(radius)?, sigma, flags)?;
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
