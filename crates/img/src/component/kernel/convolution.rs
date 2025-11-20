use std::ops::Add;

use thiserror::Error;

use crate::{
    component::{
        kernel::Kernel,
        primitive::{
            Area,
            Margin,
            Point,
            Size, SizeCreationError,
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
    #[error("margin width too big")]
    MarginWidthTooBig,
    #[error("margin height too big")]
    MarginHeightTooBig,
}

pub type ConvolutionKernelCreationResult = Result<ConvolutionKernel, ConvolutionKernelCreationError>;

/// A [`Kernel`] used to perform convolution on specified margin.
#[derive(Clone)]
pub struct ConvolutionKernel {
    size: Size,
    margin: Margin,
    buffer: Box<[f32]>,
    flags: ChannelFlags,
}

impl ConvolutionKernel {
    pub fn new(
        margin: Margin,
        buffer: Box<[f32]>,
        flags: ChannelFlags,
    ) -> ConvolutionKernelCreationResult {
        // SAFETY: 1x1 size creation should never fail
        let size = Size::new(1, 1).expect("Unexpected error in Size::new")
            .extend_by_margin(margin)
            .map_err(|e| {
                match e {
                    SizeCreationError::WidthTooBig => ConvolutionKernelCreationError::MarginWidthTooBig,
                    SizeCreationError::HeightTooBig => ConvolutionKernelCreationError::MarginHeightTooBig,
                    _ => unreachable!("unexpected error in Size::new")
                }
            })?;
        if buffer.len() != size.area() {
            return Err(ConvolutionKernelCreationError::BufferLenMissmatch);
        }

        Ok(Self { size, margin, buffer, flags })
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
    fn evaluate<S>(&self, lens: &S, point: Point) -> IndexResult<Pixel>
    where
        S: Lens<Item = In>,
    {
        let working_area = Area::from_cropped_size(
            lens.size(),
            self.margin,
        )
        .expect("failed to create working area, this is either lens or kernel bug");

        if !working_area.contains(&point) {
            return Err(IndexError::OutOfBounds);
        }

        let center = Point::new(self.margin.left(), self.margin.top()).expect("unexpected error in Point::new");

        // SAFETY: `Lens::look` always returns a value when in bounds.
        let original = lens.look(point).expect("unexpected error in Lens::look");
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
                let current = lens.look(translated).expect("unexpected error in Lens::look");
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
        self.margin
    }
}
