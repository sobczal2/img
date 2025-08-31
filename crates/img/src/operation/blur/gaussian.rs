use thiserror::Error;

use crate::{
    component::kernel::{
        self,
        gaussian::GaussianKernel,
    },
    image::Image,
    pipe::{
        self,
        FromPipe,
        Pipe,
    },
    pixel::{
        Pixel,
        PixelFlags,
    },
    primitive::size::Size,
};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to create gaussian kernel: {0}")]
    KernelCreation(#[from] kernel::gaussian::CreationError),
    #[error("failed to create kernel pipe: {0}")]
    KernelPipeCreation(#[from] pipe::kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

pub fn gaussian_blur_pipe<S>(
    source: S,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<impl Pipe<Item = Pixel>>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    let kernel = GaussianKernel::new(Size::from_radius(radius), sigma, flags)?;
    let pipe = source.kernel(kernel)?;

    Ok(pipe)
}

pub fn gaussian_blur(
    image: &Image,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<Image> {
    let pipe = gaussian_blur_pipe(image.pipe(), radius, sigma, flags)?;
    Ok(Image::from_pipe(pipe))
}

#[cfg(feature = "parallel")]
pub fn gaussian_blur_par(
    image: &Image,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<Image> {
    use crate::pipe::FromPipePar;

    let pipe = gaussian_blur_pipe(image.pipe(), radius, sigma, flags)?;
    Ok(Image::from_pipe_par(pipe))
}
