#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use thiserror::Error;

use crate::{
    component::{
        kernel::{
            self,
            mean::MeanKernel,
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

#[derive(Debug, Error)]
pub enum MeanBlurCreationError {
    #[error("failed to create mean kernel: {0}")]
    Kernel(#[from] kernel::mean::MeanKernelCreationError),
    #[error("failed to create kernel lens: {0}")]
    KernelLens(#[from] lens::kernel::CreationError),
    #[error("failed to create size: {0}")]
    Size(#[from] SizeCreationError),
}

pub type MeanBlurCreationResult<T> = std::result::Result<T, MeanBlurCreationError>;

pub fn mean_blur_lens<S>(
    source: S,
    radius: usize,
    flags: ChannelFlags,
) -> MeanBlurCreationResult<impl Lens<Item = Pixel>>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    let kernel = MeanKernel::new(Size::from_radius(radius)?, flags)?;
    let lens = source.kernel(kernel)?;
    Ok(lens)
}

pub fn mean_blur(
    image: &Image,
    radius: usize,
    flags: ChannelFlags,
) -> MeanBlurCreationResult<Image> {
    let lens = mean_blur_lens(image.lens(), radius, flags)?;
    Ok(Image::from_lens(lens))
}

#[cfg(feature = "parallel")]
pub fn mean_blur_par(
    image: &Image,
    threads: NonZeroUsize,
    radius: usize,
    flags: ChannelFlags,
) -> MeanBlurCreationResult<Image> {
    use crate::lens::FromLensPar;

    let lens = mean_blur_lens(image.lens(), radius, flags)?;
    Ok(Image::from_lens_par(lens, threads))
}
