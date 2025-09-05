use std::f32::consts::{
    E,
    PI,
};

use thiserror::Error;

use crate::{
    component::kernel::{
        self,
        Kernel,
        convolution::ConvolutionKernel,
    },
    error::IndexResult,
    lens::Lens,
    pixel::{
        Pixel,
        PixelFlags,
    },
    primitive::{
        offset::Offset,
        point::Point,
        size::Size,
    },
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("invalid sigma")]
    InvalidSigma,
    #[error("invalid convolution kernel params: {0}")]
    ConvolutionKernelError(#[from] kernel::convolution::CreationError),
}

pub type CreationResult = Result<GaussianKernel, CreationError>;

pub struct GaussianKernel {
    inner: ConvolutionKernel,
}

impl GaussianKernel {
    pub fn new(size: Size, sigma: f32, flags: PixelFlags) -> CreationResult {
        if !sigma.is_finite() || sigma <= 0f32 {
            return Err(CreationError::InvalidSigma);
        }
        let mut values = vec![0f32; size.area()];
        let center = size.middle();

        values
            .iter_mut()
            .enumerate()
            .map(|(index, value)| (Point::from_index(index, size).unwrap(), value))
            .for_each(|(point, value)| *value = gaussian_fn(point - center, sigma));

        let sum: f32 = values.iter().sum();
        let correction = 1f32 / sum;
        values.iter_mut().for_each(|value| *value *= correction);

        Ok(Self { inner: ConvolutionKernel::new(size, values, flags)? })
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
    fn apply<S>(&self, lens: &S, point: Point) -> IndexResult<Pixel>
    where
        S: Lens<Item = In>,
    {
        self.inner.apply(lens, point)
    }

    fn size(&self) -> Size {
        <ConvolutionKernel as Kernel<In, Pixel>>::size(&self.inner)
    }
}
