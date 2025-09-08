use thiserror::Error;

use crate::{
    component::{
        kernel::{
            self,
            gaussian::GaussianKernel,
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

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to create gaussian kernel: {0}")]
    KernelCreation(#[from] kernel::gaussian::CreationError),
    #[error("failed to create kernel lens: {0}")]
    KernelLensCreation(#[from] lens::kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

pub fn gaussian_blur_lens<S>(
    source: S,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<impl Lens<Item = Pixel>>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    let kernel = GaussianKernel::new(Size::from_radius(radius), sigma, flags)?;
    let lens = source.kernel(kernel)?;

    Ok(lens)
}

pub fn gaussian_blur(
    image: &Image,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<Image> {
    let lens = gaussian_blur_lens(image.lens(), radius, sigma, flags)?;
    Ok(Image::from_lens(lens))
}

#[cfg(feature = "parallel")]
pub fn gaussian_blur_par(
    image: &Image,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<Image> {
    use crate::lens::FromLensPar;

    let lens = gaussian_blur_lens(image.lens(), radius, sigma, flags)?;
    Ok(Image::from_lens_par(lens))
}
