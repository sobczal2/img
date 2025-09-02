use crate::{
    error::IndexResult,
    lens::Lens,
    primitive::{
        point::Point,
        size::Size,
    },
};

pub mod convolution;
pub mod gaussian;
pub mod mean;
pub mod sobel;

pub trait Kernel<In, Out> {
    fn apply<P>(&self, lens: &P, point: Point) -> IndexResult<Out>
    where
        P: Lens<Item = In>;

    fn size(&self) -> Size;
}
