use thiserror::Error;

use crate::{
    component::{
        kernel::{
            self,
            Kernel,
            convolution::ConvolutionKernel,
        },
        primitive::{
            Margin,
            Point,
            Size,
        },
    },
    error::IndexResult,
    lens::Lens,
    pixel::{
        ChannelFlags,
        Pixel,
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
    pub fn new(size: Size, flags: ChannelFlags) -> CreationResult {
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
    fn apply<P>(&self, lens: &P, point: Point) -> IndexResult<Pixel>
    where
        P: Lens<Item = In>,
    {
        self.inner.apply(lens, point)
    }

    fn margin(&self) -> Margin {
        <ConvolutionKernel as Kernel<In, Pixel>>::margin(&self.inner)
    }
}
