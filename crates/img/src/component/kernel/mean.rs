use crate::{
    component::kernel::Kernel,
    error::IndexResult,
    pipe::Pipe,
    pixel::Pixel,
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
    fn apply<P>(&self, _pipe: &P, _point: Point) -> IndexResult<Pixel>
    where
        P: Pipe<Item = In>,
    {
        todo!()
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
