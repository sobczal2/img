use thiserror::Error;

use crate::{
    component::kernel::{
        self,
        gaussian::GaussianKernel,
    },
    error::IndexResult,
    image::Image,
    pipe::{
        self,
        FromPipe,
        Pipe,
        kernel::KernelPipe,
    },
    pixel::{
        Pixel,
        PixelFlags,
    },
    primitive::{
        point::Point,
        size::Size,
    },
};

#[cfg(feature = "parallel")]
use crate::pipe::FromPipePar;

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to create gaussian kernel: {0}")]
    KernelCreation(#[from] kernel::gaussian::CreationError),
    #[error("failed to create kernel pipe: {0}")]
    KernelPipeCreation(#[from] pipe::kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

type Inner<S> = KernelPipe<S, GaussianKernel, Pixel>;

pub struct GaussianBlurPipe<S> {
    inner: Inner<S>,
}

impl<S> GaussianBlurPipe<S>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    pub fn new(source: S, radius: usize, sigma: f32, flags: PixelFlags) -> CreationResult<Self> {
        let kernel = GaussianKernel::new(Size::from_radius(radius), sigma, flags)?;
        Ok(Self { inner: source.kernel(kernel)? })
    }
}

impl<S> Pipe for GaussianBlurPipe<S>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    type Item = Pixel;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        self.inner.get(point)
    }

    fn size(&self) -> Size {
        self.inner.size()
    }
}

pub fn gaussian_blur(
    image: &Image,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<Image> {
    let pipe = GaussianBlurPipe::new(image.pipe(), radius, sigma, flags)?;
    Ok(Image::from_pipe(pipe))
}

#[cfg(feature = "parallel")]
pub fn gaussian_blur_par(
    image: &Image,
    radius: usize,
    sigma: f32,
    flags: PixelFlags,
) -> CreationResult<Image> {
    let pipe = GaussianBlurPipe::new(image.pipe(), radius, sigma, flags)?;
    Ok(Image::from_pipe_par(pipe))
}
