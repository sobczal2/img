use thiserror::Error;

use crate::{component::kernel::Kernel, error::{IndexResult, OutOfBoundsError}, pipe::Pipe, pixel::{Pixel, ReadPixelRgbaf32, WritePixelRgbaf32}, primitive::{area::Area, margin::Margin, point::Point, size::Size}};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("buffer len is invalid")]
    BufferLenMissmatch,
}

pub type CreationResult = Result<ConvolutionKernel, CreationError>;

pub struct ConvolutionKernel {
    size: Size,
    buffer: Box<[f32]>,
}

impl ConvolutionKernel {
    pub fn new(size: Size, buffer: impl IntoIterator<Item = f32>) -> CreationResult {
        let buffer: Vec<_> = buffer.into_iter().collect();
        if buffer.len() != size.area() {
            return Err(CreationError::BufferLenMissmatch);
        }

        Ok(Self { size, buffer: buffer.into_boxed_slice() })
    }
}

impl<In> Kernel<In, Pixel> for ConvolutionKernel
    where In: AsRef<Pixel>
{
    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<Pixel>
    where
        P: Pipe<Item = In> {
        let working_area = Area::from_cropped_size(pipe.size(), Margin::from_size(self.size));
        if !working_area.contains(point) {
            return Err(OutOfBoundsError);
        }

        let center = self.size.middle();

        let original = pipe.get(point).unwrap();
        let sum = self
            .buffer
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
