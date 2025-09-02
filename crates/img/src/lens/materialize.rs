use std::sync::Arc;

use crate::{
    error::IndexResult,
    lens::Lens,
    primitive::{
        point::Point,
        size::Size,
    },
};

pub struct MaterializeLens<T> {
    values: Arc<[T]>,
    size: Size,
}

impl<T> MaterializeLens<T> {
    pub fn new<S>(source: S) -> Self
    where
        S: Lens<Item = T>,
    {
        let size = source.size();
        let values = Arc::from_iter(source.elements());

        Self { size, values }
    }

    pub fn take_values(self) -> Arc<[T]> {
        self.values.clone()
    }
}

impl<T> Clone for MaterializeLens<T> {
    fn clone(&self) -> Self {
        Self { values: self.values.clone(), size: self.size }
    }
}

impl<T> Lens for MaterializeLens<T>
where
    T: Clone,
{
    type Item = T;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        let index = point.to_index(self.size)?;
        Ok(self.values[index].clone())
    }

    fn size(&self) -> Size {
        self.size
    }
}
