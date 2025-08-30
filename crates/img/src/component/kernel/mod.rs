use crate::{
    error::IndexResult,
    pipe::Pipe,
    primitive::{point::Point, size::Size},
};

pub mod gaussian;

pub trait Kernel<In, Out> {
    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<Out>
    where
        P: Pipe<Item = In>;

    fn size(&self) -> Size;
}
