use thiserror::Error;

use crate::{
    component::{
        kernel::{
            self, convolution::ConvolutionKernel, Kernel
        },
        primitive::{
            Margin,
            Point,
            Size, SizeCreationError,
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
pub enum MeanKernelCreationError {
    #[error("invalid convolution kernel params: {0}")]
    ConvolutionKernelError(#[from] kernel::convolution::ConvolutionKernelCreationError),
    #[error("margin width too big")]
    MarginWidthTooBig,
    #[error("margin height too big")]
    MarginHeightTooBig,
}

pub type MeanKernelCreationResult = Result<MeanKernel, MeanKernelCreationError>;

#[derive(Clone)]
pub struct MeanKernel {
    inner: ConvolutionKernel,
}

impl MeanKernel {
    pub fn new(margin: Margin, flags: ChannelFlags) -> MeanKernelCreationResult {
        // SAFETY: 1x1 size creation should never change
        let size = Size::new(1, 1)
            .expect("Unexpected error in Size::new")
            .extend_by_margin(margin)
            .map_err(|e| {
                match e {
                    SizeCreationError::WidthTooBig => MeanKernelCreationError::MarginWidthTooBig,
                    SizeCreationError::HeightTooBig => MeanKernelCreationError::MarginHeightTooBig,
                    _ => unreachable!("unexpected error in Size::new")
                }
            })?;
        Ok(Self {
            inner: ConvolutionKernel::new(
                margin,
                vec![1f32 / size.area() as f32; size.area()].into_boxed_slice(),
                flags,
            )?,
        })
    }
}

impl<In> Kernel<In, Pixel> for MeanKernel
where
    In: AsRef<Pixel>,
{
    fn evaluate<P>(&self, lens: &P, point: Point) -> IndexResult<Pixel>
    where
        P: Lens<Item = In>,
    {
        self.inner.evaluate(lens, point)
    }

    fn margin(&self) -> Margin {
        <ConvolutionKernel as Kernel<In, Pixel>>::margin(&self.inner)
    }
}
