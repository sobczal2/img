use thiserror::Error;

use crate::{
    component::kernel::{
        self,
        Kernel,
        convolution::ConvolutionKernel,
    },
    error::IndexResult,
    pipe::Pipe,
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
    #[error("invalid convolution kernel params: {0}")]
    ConvolutionKernelError(#[from] kernel::convolution::CreationError),
}

pub type CreationResult = Result<MeanKernel, CreationError>;

#[derive(Clone)]
pub struct MeanKernel {
    inner: ConvolutionKernel,
}

impl MeanKernel {
    pub fn new(size: Size, flags: PixelFlags) -> CreationResult {
        Ok(Self {
            inner: ConvolutionKernel::new(
                size,
                vec![1f32 / size.area() as f32; size.area()],
                flags,
            )?,
        })
    }
}

impl<In> Kernel<In, Pixel> for MeanKernel
where
    In: AsRef<Pixel>,
{
    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<Pixel>
    where
        P: Pipe<Item = In>,
    {
        self.inner.apply(pipe, point)
    }

    fn size(&self) -> Size {
        <ConvolutionKernel as Kernel<In, Pixel>>::size(&self.inner)
    }
}
