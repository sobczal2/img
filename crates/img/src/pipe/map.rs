use crate::{
    error::IndexResult,
    pipe::Pipe,
    primitive::{point::Point, size::Size},
};

pub struct MapPipe<P, F> {
    source: P,
    f: F,
}

impl<P, F> MapPipe<P, F> {
    pub fn new(source: P, f: F) -> Self {
        Self { source, f }
    }
}

impl<T, P: Pipe, F> Pipe for MapPipe<P, F>
where
    F: Fn(P::Item) -> T,
{
    type Item = T;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        Ok((self.f)(self.source.get(point)?))
    }

    fn size(&self) -> Size {
        self.source.size()
    }
}
