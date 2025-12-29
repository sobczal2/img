use crate::{
    component::primitive::{
        Margin,
        Point,
    },
    error::IndexResult,
    lens::Lens,
    prelude::Size,
};

pub mod convolution;
pub mod gaussian;
pub mod identity;
pub mod mean;
pub mod median;
pub mod sobel;

pub trait Kernel<In, Out> {
    fn apply<S>(&self, source: &S, point: Point) -> IndexResult<Out>
    where
        S: Lens<Item = In>;

    fn margin(&self) -> Margin;
}

pub fn kernel_margin_for_size(size: Size) -> Margin {
    let (left, right) = if size.width().is_multiple_of(2) {
        (size.width() / 2, size.width() / 2 - 1)
    } else {
        (size.width() / 2, size.width() / 2)
    };

    let (top, bottom) = if size.height().is_multiple_of(2) {
        (size.height() / 2, size.height() / 2 - 1)
    } else {
        (size.height() / 2, size.height() / 2)
    };

    // SAFETY: all parameters are halves of some size which is guaranted to be
    // less than or equal to DIMENSION_MAX.
    Margin::new(top, right, bottom, left).expect("unexpected error in Margin::new")
}
