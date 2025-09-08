use thiserror::Error;

use crate::{
    component::{
        kernel::{
            self,
            mean::MeanKernel,
        },
        primitive::Size,
    },
    image::Image,
    lens::{
        self,
        FromLens,
        Lens,
    },
    pixel::{
        Pixel,
        PixelFlags,
    },
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to create mean kernel: {0}")]
    KernelCreation(#[from] kernel::mean::CreationError),
    #[error("failed to create kernel lens: {0}")]
    KernelLensCreation(#[from] lens::kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

pub fn mean_blur_lens<S>(
    source: S,
    radius: usize,
    flags: PixelFlags,
) -> CreationResult<impl Lens<Item = Pixel>>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    let kernel = MeanKernel::new(Size::from_radius(radius), flags)?;
    let lens = source.kernel(kernel)?;
    Ok(lens)
}

pub fn mean_blur(image: &Image, radius: usize, flags: PixelFlags) -> CreationResult<Image> {
    let lens = mean_blur_lens(image.lens(), radius, flags)?;
    Ok(Image::from_lens(lens))
}

#[cfg(feature = "parallel")]
pub fn mean_blur_par(image: &Image, radius: usize, flags: PixelFlags) -> CreationResult<Image> {
    use crate::lens::FromLensPar;

    let lens = mean_blur_lens(image.lens(), radius, flags)?;
    Ok(Image::from_lens_par(lens))
}
