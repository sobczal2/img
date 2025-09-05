use crate::{
    error::IndexResult,
    lens::Lens,
    primitive::{
        point::Point,
        size::Size,
    },
};

#[derive(Clone)]
pub struct ClonedLens<S> {
    source: S,
}

impl<S> ClonedLens<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

impl<'a, S, T> Lens for ClonedLens<S>
where
    S: Lens<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        self.source.look(point).cloned()
    }

    fn size(&self) -> Size {
        self.source.size()
    }
}
