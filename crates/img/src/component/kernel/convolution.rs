use std::ops::Add;

use thiserror::Error;

use crate::{
    component::{
        kernel::{
            Kernel,
            kernel_margin_for_size,
        },
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
pub enum ConvolutionKernelCreationError {
    #[error("buffer len is invalid")]
    BufferLenMissmatch,
}

pub type ConvolutionKernelCreationResult =
    Result<ConvolutionKernel, ConvolutionKernelCreationError>;

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
    ) -> ConvolutionKernelCreationResult {
        let buffer: Box<[_]> = buffer.into_iter().collect();
        if buffer.len() != size.area() {
            return Err(ConvolutionKernelCreationError::BufferLenMissmatch);
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
    fn apply<S>(&self, source: &S, point: Point) -> IndexResult<Pixel>
    where
        S: Lens<Item = In>,
    {
        let working_area = Area::from_cropped_size(
            source.size(),
            <ConvolutionKernel as Kernel<In, Pixel>>::margin(self),
        )
        .expect("failed to create working area, this is either lens or kernel bug");

        if !working_area.contains(&point) {
            return Err(IndexError::OutOfBounds);
        }

        let center = self.size.middle();

        // SAFETY: `Lens::look` always returns a value when in bounds.
        let original = source.look(point).expect("unexpected error in Lens::look");
        let sum = self
            .buffer
            .iter()
            .enumerate()
            .map(|(index, value)| {
                (
                    // SAFETY: index comes from the buffer of size used, so it is always in bounds.
                    Point::from_index(index, self.size)
                        .expect("unexpected error in Point::from_index"),
                    value,
                )
            })
            .map(|(kernel_point, value)| {
                let offset = center - kernel_point;
                // SAFETY: translated point always in bounds after previous checks.
                let translated =
                    point.translate(offset).expect("unexpected error in Point::translate");
                // SAFETY: `Lens::look` always returns a value when in bounds.
                let current = source.look(translated).expect("unexpected error in Lens::look");
                let pixel = current.as_ref();

                IntermediatePixel(
                    value * pixel.r_f32(),
                    value * pixel.g_f32(),
                    value * pixel.b_f32(),
                    value * pixel.a_f32(),
                )
            })
            .fold(IntermediatePixel(0f32, 0f32, 0f32, 0f32), |acc, item| acc + item);

        let mut px = *original.as_ref();
        px.set_with_flags_f32(sum.0, sum.1, sum.2, sum.3, self.flags);

        Ok(px)
    }

    fn margin(&self) -> Margin {
        kernel_margin_for_size(self.size)
    }
}
