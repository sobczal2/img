use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::{
        IndexError,
        IndexResult,
    },
    lens::Lens,
};

pub struct ValueLens<T> {
    value: T,
    size: Size,
}

impl<T> ValueLens<T> {
    pub fn new(value: T, size: Size) -> Self {
        Self { value, size }
    }
}

impl<T> Lens for ValueLens<T>
where
    T: Clone,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        if self.size.contains(&point) {
            return Ok(self.value.clone());
        }

        Err(IndexError::OutOfBounds)
    }

    fn size(&self) -> Size {
        self.size
    }
}
