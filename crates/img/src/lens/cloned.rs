use crate::{
    error::IndexResult,
    lens::Lens,
    primitive::{
        point::Point,
        size::Size,
    },
};

#[derive(Clone)]
pub struct ClonedLens<P> {
    source: P,
}

impl<P> ClonedLens<P> {
    pub fn new(source: P) -> Self {
        Self { source }
    }
}

impl<'a, S, P> Lens for ClonedLens<P>
where
    P: Lens<Item = &'a S>,
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
