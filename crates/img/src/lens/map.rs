use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::IndexResult,
    lens::Lens,
};

pub struct MapLens<S, F> {
    source: S,
    f: F,
}

impl<S, F> MapLens<S, F> {
    pub fn new(source: S, f: F) -> Self {
        Self { source, f }
    }
}

impl<T, S, F> Lens for MapLens<S, F>
where
    S: Lens,
    F: Fn(S::Item) -> T,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        Ok((self.f)(self.source.look(point)?))
    }

    fn size(&self) -> Size {
        self.source.size()
    }
}
