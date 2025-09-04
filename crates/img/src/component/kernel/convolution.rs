use std::ops::Add;

use thiserror::Error;

use crate::{
    component::kernel::Kernel,
    error::{
        IndexResult,
        OutOfBoundsError,
    },
    lens::Lens,
    pixel::{
        Pixel,
        PixelFlags,
        ReadPixelRgbaf32,
        WritePixelRgbaf32,
    },
    primitive::{
        area::Area,
        margin::Margin,
        point::Point,
        size::Size,
    },
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("buffer len is invalid")]
    BufferLenMissmatch,
}

pub type CreationResult = Result<ConvolutionKernel, CreationError>;

#[derive(Clone)]
pub struct ConvolutionKernel {
    size: Size,
    buffer: Box<[f32]>,
    flags: PixelFlags,
}

impl ConvolutionKernel {
    pub fn new(
        size: Size,
        buffer: impl IntoIterator<Item = f32>,
        flags: PixelFlags,
    ) -> CreationResult {
        let buffer: Vec<_> = buffer.into_iter().collect();
        if buffer.len() != size.area() {
            return Err(CreationError::BufferLenMissmatch);
        }

        Ok(Self { size, buffer: buffer.into_boxed_slice(), flags })
    }
}

#[derive(Default)]
struct IntermediatePixel(f32, f32, f32, f32);

impl Add for IntermediatePixel {
    type Output = IntermediatePixel;

    fn add(self, rhs: Self) -> Self::Output {
        IntermediatePixel(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl<In> Kernel<In, Pixel> for ConvolutionKernel
where
    In: AsRef<Pixel>,
{
    fn apply<P>(&self, lens: &P, point: Point) -> IndexResult<Pixel>
    where
        P: Lens<Item = In>,
    {
        // TODO: This isn't a out of bounds error, but currently we have no way to return something
        // else here
        let working_area = Area::from_cropped_size(lens.size(), Margin::from_size(self.size))
            .map_err(|_| OutOfBoundsError)?;
        if !working_area.contains(&point) {
            return Err(OutOfBoundsError);
        }

        let center = self.size.middle();

        let original = lens.look(point).unwrap();
        let sum = self
            .buffer
            .iter()
            .enumerate()
            .map(|(index, value)| (Point::from_index(index, self.size).unwrap(), value))
            .map(|(kernel_point, value)| {
                let offset = center - kernel_point;
                let current = lens.look(point.translate(offset).unwrap()).unwrap();
                let pixel = current.as_ref();

                IntermediatePixel(
                    value * pixel.r_f32(),
                    value * pixel.g_f32(),
                    value * pixel.b_f32(),
                    value * pixel.a_f32(),
                )
            })
            .reduce(|l, r| l + r)
            .unwrap();

        let mut px = *original.as_ref();
        px.set_with_flags_f32(sum.0, sum.1, sum.2, sum.3, self.flags);

        Ok(px)
    }

    fn size(&self) -> Size {
        self.size
    }
}
