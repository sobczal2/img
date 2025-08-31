use crate::{
    component::kernel::Kernel,
    error::{IndexResult, OutOfBoundsError},
    pipe::Pipe,
    pixel::{Pixel, ReadPixelRgbaf32, WritePixelRgbaf32},
    primitive::{area::Area, margin::Margin, point::Point, size::Size},
};

pub struct MeanKernel {
    size: Size,
    value: f32,
}

impl MeanKernel {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            value: 1f32 / size.area() as f32,
        }
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
        let working_area = Area::from_cropped_size(pipe.size(), Margin::from_size(self.size));
        if !working_area.contains(point) {
            return Err(OutOfBoundsError);
        }

        let center = self.size.middle();

        let original = pipe.get(point).unwrap();
        let sum = (0..self.size.area())
            .map(|index| Point::from_index(index, self.size).unwrap())
            .map(|kernel_point| {
                let offset = center - kernel_point;
                let current = pipe.get(point.offset_by(offset).unwrap()).unwrap();
                let pixel = current.as_ref();

                (
                    self.value * pixel.r_f32(),
                    self.value * pixel.g_f32(),
                    self.value * pixel.b_f32(),
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

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::*;

    #[test]
    fn value() -> Result<(), Box<dyn Error>> {
        assert_eq!(MeanKernel::new(Size::from_usize(1, 1)?).value, 1f32);
        assert_eq!(MeanKernel::new(Size::from_usize(1, 10)?).value, 0.1f32);
        assert_eq!(MeanKernel::new(Size::from_usize(10, 1)?).value, 0.1f32);
        assert_eq!(MeanKernel::new(Size::from_usize(10, 10)?).value, 0.01f32);

        Ok(())
    }
}
