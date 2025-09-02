use crate::{
    error::IndexResult,
    pipe::Pipe,
    primitive::{
        point::Point,
        size::Size,
    },
};

#[derive(Clone)]
pub struct RemapPipe<P, F> {
    source: P,
    f: F,
    size: Size,
}

impl<P, F> RemapPipe<P, F> {
    pub fn new(source: P, f: F, size: Size) -> Self {
        Self { source, f, size }
    }
}

// TODO: consider remapping point here instead of callee
impl<T, P: Pipe, F> Pipe for RemapPipe<P, F>
where
    F: Fn(&P, Point) -> T,
{
    type Item = T;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        Ok((self.f)(&self.source, point))
    }

    fn size(&self) -> Size {
        self.size
    }
}
