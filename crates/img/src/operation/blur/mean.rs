use thiserror::Error;

use crate::{
    component::kernel::{
        self,
        mean::MeanKernel,
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

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to create mean kernel: {0}")]
    KernelCreation(#[from] kernel::mean::CreationError),
    #[error("failed to create kernel pipe: {0}")]
    KernelPipeCreation(#[from] pipe::kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

type Inner<S> = KernelPipe<S, MeanKernel, Pixel>;

pub struct MeanBlurPipe<S> {
    inner: Inner<S>,
}

impl<S> MeanBlurPipe<S>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    pub fn new(source: S, radius: usize, flags: PixelFlags) -> CreationResult<Self> {
        let kernel = MeanKernel::new(Size::from_radius(radius), flags)?;
        Ok(Self { inner: source.kernel(kernel)? })
    }
}

impl<S> Pipe for MeanBlurPipe<S>
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

pub fn mean_blur(image: &Image, radius: usize, flags: PixelFlags) -> CreationResult<Image> {
    let pipe = MeanBlurPipe::new(image.pipe(), radius, flags)?;
    Ok(Image::from_pipe(pipe))
}

#[cfg(feature = "parallel")]
pub fn mean_blur_par(image: &Image, radius: usize, flags: PixelFlags) -> CreationResult<Image> {
    use crate::pipe::FromPipePar;

    let pipe = MeanBlurPipe::new(image.pipe(), radius, flags)?;
    Ok(Image::from_pipe_par(pipe))
}
