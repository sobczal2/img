use std::num::NonZeroUsize;

use thiserror::Error;

use crate::{
    component::{
        kernel::median::MedianKernel,
        primitive::SizeCreationError,
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
    prelude::Size,
};

#[derive(Debug, Error)]
pub enum MedianBlurCreationError {
    #[error("failed to create kernel lens: {0}")]
    KernelLens(#[from] lens::kernel::CreationError),
    #[error("failed to create size: {0}")]
    Size(#[from] SizeCreationError),
}

pub type MedianBlurCreationResult<T> = std::result::Result<T, MedianBlurCreationError>;

pub fn median_blur(
    image: &Image,
    radius: usize,
    flags: ChannelFlags,
) -> MedianBlurCreationResult<Image> {
    let lens = median_blur_lens(image.lens(), radius, flags)?;
    Ok(Image::from_lens(lens))
}

pub fn median_blur_lens<S>(
    source: S,
    radius: usize,
    flags: ChannelFlags,
) -> MedianBlurCreationResult<impl Lens<Item = Pixel>>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    Ok(source
        .map(|p| *p.as_ref())
        .materialize()
        .kernel(MedianKernel::new(Size::from_radius(radius)?, flags))?)
}

#[cfg(feature = "parallel")]
pub fn median_blur_par(
    image: &Image,
    threads: NonZeroUsize,
    radius: usize,
    flags: ChannelFlags,
) -> MedianBlurCreationResult<Image> {
    use crate::lens::FromLensPar;

    let lens = median_blur_lens_par(image.lens(), threads, radius, flags)?;
    Ok(Image::from_lens_par(lens, threads))
}

#[cfg(feature = "parallel")]
pub fn median_blur_lens_par<S>(
    source: S,
    threads: NonZeroUsize,
    radius: usize,
    flags: ChannelFlags,
) -> MedianBlurCreationResult<impl Lens<Item = Pixel>>
where
    S: Lens + Send + Sync,
    S::Item: AsRef<Pixel>,
{
    let size = Size::from_radius(radius)?;
    Ok(source
        .map(|p| *p.as_ref())
        .materialize_par(threads)
        .kernel_par(move || MedianKernel::new(size, flags))?)
}
