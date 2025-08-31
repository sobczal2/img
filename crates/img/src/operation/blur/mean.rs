use thiserror::Error;

use crate::{
    component::kernel::{
        self,
        mean::MeanKernel,
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

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("failed to create mean kernel: {0}")]
    KernelCreation(#[from] kernel::mean::CreationError),
    #[error("failed to create kernel pipe: {0}")]
    KernelPipeCreation(#[from] pipe::kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

pub fn mean_blur_pipe<S>(
    source: S,
    radius: usize,
    flags: PixelFlags,
) -> CreationResult<impl Pipe<Item = Pixel>>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    let kernel = MeanKernel::new(Size::from_radius(radius), flags)?;
    let pipe = source.kernel(kernel)?;
    Ok(pipe)
}

pub fn mean_blur(image: &Image, radius: usize, flags: PixelFlags) -> CreationResult<Image> {
    let pipe = mean_blur_pipe(image.pipe(), radius, flags)?;
    Ok(Image::from_pipe(pipe))
}

#[cfg(feature = "parallel")]
pub fn mean_blur_par(image: &Image, radius: usize, flags: PixelFlags) -> CreationResult<Image> {
    use crate::pipe::FromPipePar;

    let pipe = mean_blur_pipe(image.pipe(), radius, flags)?;
    Ok(Image::from_pipe_par(pipe))
}
