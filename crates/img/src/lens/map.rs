use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::IndexResult,
    lens::Lens,
};

/// A [`Lens`] that maps values of `source` with `f`.
///
/// This `struct` is created by the [`map`] mathod on [`Lens`]. See its documentation for more.
///
/// [`map`]: Lens::map
#[derive(Clone)]
pub struct MapLens<S, F> {
    source: S,
    f: F,
}

impl<S, F> MapLens<S, F> {
    pub(super) fn new(source: S, f: F) -> Self {
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
        self.source.look(point).map(&self.f)
    }

    fn size(&self) -> Size {
        self.source.size()
    }
}
