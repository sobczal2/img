use std::{
    borrow::Borrow,
    f32::consts::{E, PI},
};

use crate::{
    component::kernel::Kernel,
    error::IndexResult,
    pipe::Pipe,
    pixel::{Pixel, ReadPixelRgbaf32, WritePixelRgbaf32},
    primitive::{offset::Offset, point::Point, size::Size},
};

pub struct GaussianKernel {
    size: Size,
    values: Box<[f32]>,
}

impl GaussianKernel {
    pub fn new(size: Size, sigma: f32) -> Self {
        let mut values = vec![0f32; size.area()];
        let center = size.center();

        values
            .iter_mut()
            .enumerate()
            .map(|(index, value)| (Point::from_index(index, size).unwrap(), value))
            .for_each(|(point, value)| *value = gaussian_fn(point - center, sigma));

        Self {
            size,
            values: values.into_boxed_slice(),
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
    // TODO: Error handling, perhabs some abstraction
    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<Pixel>
    where
        P: Pipe<Item = In>,
    {
        let center = self.size.center();

        let original = pipe.get(point).unwrap();
        let sum = self
            .values
            .iter()
            .enumerate()
            .map(|(index, value)| (Point::from_index(index, self.size).unwrap(), value))
            .map(|(kernel_point, value)| {
                let offset = center - kernel_point;
                let current = pipe.get(point.offset_by(offset).unwrap()).unwrap();
                let pixel = current.as_ref();

                (
                    value * pixel.r_f32(),
                    value * pixel.g_f32(),
                    value * pixel.b_f32(),
                )
            })
            .fold((0f32, 0f32, 0f32), |acc, item| {
                (acc.0 + item.0, acc.1 + item.1, acc.2 + item.2)
            });

        let mut px = Pixel::zero();
        px.set_r_f32(sum.0);
        px.set_g_f32(sum.1);
        px.set_b_f32(sum.2);
        px.set_a(original.as_ref().a());

        Ok(px)
    }

    fn size(&self) -> Size {
        self.size
    }
}
