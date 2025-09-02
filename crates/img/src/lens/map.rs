use crate::{
    error::IndexResult,
    lens::Lens,
    primitive::{
        point::Point,
        size::Size,
    },
};

pub struct MapLens<P, F> {
    source: P,
    f: F,
}

impl<P, F> MapLens<P, F> {
    pub fn new(source: P, f: F) -> Self {
        Self { source, f }
    }
}

impl<T, P: Lens, F> Lens for MapLens<P, F>
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
