use crate::{
    error::IndexResult,
    pipe::Pipe,
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
    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<Out>
    where
        P: Pipe<Item = In>;

    fn size(&self) -> Size;
}

