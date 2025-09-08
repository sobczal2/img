use std::{
    iter::from_fn,
    sync::Arc,
};

use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::IndexResult,
    lens::Lens,
};

pub struct MaterializeLens<T> {
    values: Arc<[Option<T>]>,
    size: Size,
}

impl<T> MaterializeLens<T> {
    pub fn new<S>(source: S) -> Self
    where
        S: Lens<Item = T>,
    {
        let size = source.size();
        let values = Arc::from_iter(source.elements().map(|e| Some(e)));

        Self { size, values }
    }

    #[cfg(feature = "parallel")]
    pub fn new_par<S>(source: S) -> Self
    where
        S: Lens<Item = T> + Send + Sync,
        T: Send,
    {
        use std::thread;

        let size = source.size();
        let cpus = num_cpus::get();
        let chunk_size = (size.area() as f32 / cpus as f32).ceil() as usize;

        let mut values = Box::from_iter(from_fn(|| Some(None)).take(size.area()));

        let value_chunks = values.chunks_mut(chunk_size);

        thread::scope(|scope| {
            value_chunks.enumerate().for_each(|(index, chunk)| {
                let source = &source;
                scope.spawn(move || {
                    let starting_index = index * chunk_size;
                    chunk.iter_mut().enumerate().for_each(|(index, value)| {
                        let point = Point::from_index(starting_index + index, size).unwrap();
                        *value = Some(source.look(point).unwrap());
                    });
                });
            });
        });

        Self { size, values: values.into() }
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

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        let index = point.index(self.size)?;
        Ok(self.values[index].clone().unwrap())
    }

    fn size(&self) -> Size {
        self.size
    }
}
