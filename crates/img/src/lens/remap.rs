use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::IndexResult,
    lens::Lens,
};

#[derive(Clone)]
pub struct RemapLens<S, F> {
    source: S,
    f: F,
    size: Size,
}

impl<S, F> RemapLens<S, F> {
    pub fn new(source: S, f: F, size: Size) -> Self {
        Self { source, f, size }
    }
}

impl<T, S, F> Lens for RemapLens<S, F>
where
    S: Lens,
    F: Fn(&S, Point) -> IndexResult<T>,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        (self.f)(&self.source, point)
    }

    fn size(&self) -> Size {
        self.size
    }
}
