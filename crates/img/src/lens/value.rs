use crate::{
    error::{
        IndexResult,
        OutOfBoundsError,
    },
    lens::Lens,
    primitive::{
        point::Point,
        size::Size,
    },
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

        Err(OutOfBoundsError)
    }

    fn size(&self) -> Size {
        self.size
    }
}
