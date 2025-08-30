use crate::{
    error::IndexResult,
    pipe::Pipe,
    primitive::{point::Point, size::Size},
};

pub struct ClonedPipe<P> {
    source: P,
}

impl<P> ClonedPipe<P> {
    pub fn new(source: P) -> Self {
        Self { source }
    }
}

impl<'a, S, P> Pipe for ClonedPipe<P>
where
    P: Pipe<Item = &'a S>,
    S: Clone + 'a,
{
    type Item = S;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        self.source.get(point).cloned()
    }

    fn size(&self) -> Size {
        self.source.size()
    }
}
