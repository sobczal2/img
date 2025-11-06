#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;
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
    lens::{
        FromLens,
        FromLensPar,
        Lens,
    },
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
    pub fn new_par<S>(source: S, threads: NonZeroUsize) -> Self
    where
        S: Lens<Item = T> + Send + Sync,
        T: Send,
    {
        use std::thread;

        let size = source.size();
        let chunk_size = (size.area() as f32 / threads.get() as f32).ceil() as usize;

        let mut values = Box::from_iter(from_fn(|| Some(None)).take(size.area()));

        let value_chunks = values.chunks_mut(chunk_size);

        thread::scope(|scope| {
            value_chunks.enumerate().for_each(|(index, chunk)| {
                let source = &source;
                scope.spawn(move || {
                    let starting_index = index * chunk_size;
                    chunk.iter_mut().enumerate().for_each(|(index, value)| {
                        // SAFETY: all starting_index + index will be in bounds since it enumerates
                        // over the lens that it is indexing.
                        let point = Point::from_index(starting_index + index, size)
                            .expect("Point::from_index");
                        // SAFETY: `Lens::look` is guaranteed to return Ok if point is in bounds,
                        // and point is guaranted to be in bounds because of the check above.
                        *value =
                            Some(source.look(point).expect("unexpected error from Lens::look"));
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

        // SAFETY: index is guaranteed to be valid thanks to the check above, and value
        // has to be initialized since it went through constructor.
        Ok(self.values[index].clone().expect("unexpected empty value encountered"))
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl<T> FromLens<T> for MaterializeLens<T> {
    fn from_lens<S>(source: S) -> Self
    where
        S: Lens<Item = T>,
    {
        MaterializeLens::new(source)
    }
}

#[cfg(feature = "parallel")]
impl<T> FromLensPar<T> for MaterializeLens<T> {
    fn from_lens_par<S>(source: S, threads: NonZeroUsize) -> Self
    where
        S: Lens<Item = T> + Send + Sync,
        S::Item: Send,
    {
        MaterializeLens::new_par(source, threads)
    }
}
