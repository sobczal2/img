use std::marker::PhantomData;

use crate::{error::IndexResult, pipe::Pipe, primitives::point::Point};

pub struct MapPipe<S, T, P, F>
where
    P: Pipe<Item = S>,
    F: Fn(S) -> T,
{
    source: P,
    r#fn: F,
    _phantom_s: PhantomData<S>,
    _phantom_t: PhantomData<T>,
}

impl<S, T, V, F> MapPipe<S, T, V, F>
where
    V: Pipe<Item = S>,
    F: Fn(S) -> T,
{
    pub fn new(source: V, r#fn: F) -> Self {
        Self {
            source,
            r#fn,
            _phantom_s: Default::default(),
            _phantom_t: Default::default(),
        }
    }
}

impl<S, T, V, F> Pipe for MapPipe<S, T, V, F>
where
    V: Pipe<Item = S>,
    F: Fn(S) -> T,
{
    type Item = T;

    fn get(&self, point: Point) -> IndexResult<T> {
        Ok((self.r#fn)(self.source.get(point)?))
    }

    fn size(&self) -> crate::primitives::size::Size {
        self.source.size()
    }
}
