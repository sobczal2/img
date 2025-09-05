use crate::{
    error::IndexResult,
    lens::Lens,
    primitive::{
        margin::Margin,
        point::Point,
    },
};

pub mod convolution;
pub mod gaussian;
pub mod mean;
pub mod sobel;

pub trait Kernel<In, Out> {
    fn apply<S>(&self, source: &S, point: Point) -> IndexResult<Out>
    where
        S: Lens<Item = In>;

    fn margin(&self) -> Margin;
}
