use std::ops::Add;

use thiserror::Error;

use crate::{
    component::{
        kernel::Kernel,
        primitive::{
            Area,
            Margin,
            Point,
            Size,
        },
    },
    error::{
        IndexError,
        IndexResult,
    },
    lens::Lens,
    pixel::{
        ChannelFlags,
        Pixel,
        PixelRgbaf32,
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
    flags: ChannelFlags,
}

impl ConvolutionKernel {
    pub fn new(
        size: Size,
        buffer: impl IntoIterator<Item = f32>,
        flags: ChannelFlags,
    ) -> CreationResult {
        let buffer: Box<[_]> = buffer.into_iter().collect();
        if buffer.len() != size.area() {
            return Err(CreationError::BufferLenMissmatch);
        }

        Ok(Self { size, buffer, flags })
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
    fn apply<S>(&self, lens: &S, point: Point) -> IndexResult<Pixel>
    where
        S: Lens<Item = In>,
    {
        let working_area = Area::from_cropped_size(
            lens.size(),
            <ConvolutionKernel as Kernel<In, Pixel>>::margin(self),
        )
        .expect("failed to create working area, this is either lens or kernel bug");

        if !working_area.contains(&point) {
            return Err(IndexError::OutOfBounds);
        }

        let center = self.size.middle();

        // SAFETY: `Lens::look` always returns a value when in bounds.
        let original = lens.look(point).expect("unexpected error in Lens::look");
        let sum = self
            .buffer
            .iter()
            .enumerate()
            .map(|(index, value)| (
                // SAFETY: index comes from the buffer of size used, so it is always in bounds.
                Point::from_index(index, self.size).expect("unexpected error in Point::from_index"),
                value
            )
            )
            .map(|(kernel_point, value)| {
                let offset = center - kernel_point;
                // SAFETY: translated point always in bounds after previous checks.
                let translated = point.translate(offset).expect("unexpected error in Point::translate");
                // SAFETY: `Lens::look` always returns a value when in bounds.
                let current = lens.look(translated).expect("unexpected error in Lens::look");
                let pixel = current.as_ref();

                IntermediatePixel(
                    value * pixel.r_f32(),
                    value * pixel.g_f32(),
                    value * pixel.b_f32(),
                    value * pixel.a_f32(),
                )
            })
            .reduce(|l, r| l + r)
            // SAFETY: iterator is never empty so reduce always returns `Some`.
            .expect("unexpected error in reduce");

        let mut px = *original.as_ref();
        px.set_with_flags_f32(sum.0, sum.1, sum.2, sum.3, self.flags);

        Ok(px)
    }

    fn margin(&self) -> Margin {
        let (left, right) = if self.size.width().is_multiple_of(2) {
            (self.size.width() / 2, self.size.width() / 2 - 1)
        } else {
            (self.size.width() / 2, self.size.width() / 2)
        };

        let (top, bottom) = if self.size.height().is_multiple_of(2) {
            (self.size.height() / 2, self.size.height() / 2 - 1)
        } else {
            (self.size.height() / 2, self.size.height() / 2)
        };

        // SAFETY: all parameters are halves of some size which is guaranted to be
        // less than or equal to DIMENSION_MAX.
        Margin::new(top, right, bottom, left).expect("unexpected error in Margin::new")
    }
}
