use std::f32::consts::{E, PI};

use crate::{
    component::kernel::{convolution::ConvolutionKernel, Kernel},
    error::IndexResult,
    pipe::Pipe,
    pixel::Pixel,
    primitive::{offset::Offset, point::Point, size::Size},
};

pub struct GaussianKernel {
    inner: ConvolutionKernel,
}

impl GaussianKernel {
    pub fn new(size: Size, sigma: f32) -> Self {
        let mut values = vec![0f32; size.area()];
        let center = size.middle();

        values
            .iter_mut()
            .enumerate()
            .map(|(index, value)| (Point::from_index(index, size).unwrap(), value))
            .for_each(|(point, value)| *value = gaussian_fn(point - center, sigma));

        Self {
            inner: ConvolutionKernel::new(size, values).unwrap()
        }
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
