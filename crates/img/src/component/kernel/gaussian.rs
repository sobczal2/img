use std::f32::consts::{
    E,
    PI,
};

use thiserror::Error;

use crate::{
    component::{
        kernel::{
            convolution::{ConvolutionKernel, ConvolutionKernelCreationError}, Kernel
        },
        primitive::{
            Margin,
            Offset,
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
pub enum GaussianKernelCreationError {
    #[error("invalid sigma")]
    InvalidSigma,
    #[error("invalid convolution kernel params: {0}")]
    ConvolutionKernelError(#[from] ConvolutionKernelCreationError),
    #[error("margin width too big")]
    MarginWidthTooBig,
    #[error("margin height too big")]
    MarginHeightTooBig,
}

pub type GaussianKernelCreationResult = Result<GaussianKernel, GaussianKernelCreationError>;

#[derive(Clone)]
pub struct GaussianKernel {
    inner: ConvolutionKernel,
}

impl GaussianKernel {
    pub fn new(margin: Margin, sigma: f32, flags: ChannelFlags) -> GaussianKernelCreationResult {
        if !sigma.is_finite() || sigma <= 0f32 {
            return Err(GaussianKernelCreationError::InvalidSigma);
        }
        // SAFETY: 1x1 size creation should never fail
        let size = Size::new(1, 1)
            .expect("unexpected error in Size::new")
            .extend_by_margin(margin)
            .map_err(|e| {
                match e {
                    SizeCreationError::WidthTooBig => GaussianKernelCreationError::MarginWidthTooBig,
                    SizeCreationError::HeightTooBig => GaussianKernelCreationError::MarginHeightTooBig,
                    _ => unreachable!("unexpected error in Size::extend_by_margin")
                }
            })?;
        let mut values = vec![0f32; size.area()];
        let center = Point::new(margin.left(), margin.top())
            // SAFETY: margin.left and margin.top are guaranteed to be < DIMENSION_MAX
            .expect("unexpected error in Point::new");

        values
            .iter_mut()
            .enumerate()
            .map(|(index, value)| {
                (
                    // SAFETY: we construct the index from the area of size passed to the
                    // index creation, so it is always in bounds.
                    Point::from_index(index, size).expect("unexpected error in Point::from_index"),
                    value,
                )
            })
            .for_each(|(point, value)| *value = gaussian_fn(point - center, sigma));

        let sum: f32 = values.iter().sum();
        let correction = 1f32 / sum;
        values.iter_mut().for_each(|value| *value *= correction);

        Ok(Self { inner: ConvolutionKernel::new(margin, values.into_boxed_slice(), flags)? })
    }
}

fn gaussian_fn(offset: Offset, sigma: f32) -> f32 {
    let sigma_2 = sigma * sigma;
    let x_2 = (offset.x() * offset.x()) as f32;
    let y_2 = (offset.y() * offset.y()) as f32;

    (1f32 / (2f32 * PI * sigma_2)) * E.powf(-(x_2 + y_2) / (2f32 * sigma_2))
}

impl<In> Kernel<In, Pixel> for GaussianKernel
where
    In: AsRef<Pixel>,
{
    fn evaluate<S>(&self, lens: &S, point: Point) -> IndexResult<Pixel>
    where
        S: Lens<Item = In>,
    {
        self.inner.evaluate(lens, point)
    }

    fn margin(&self) -> Margin {
        <ConvolutionKernel as Kernel<In, Pixel>>::margin(&self.inner)
    }
}
