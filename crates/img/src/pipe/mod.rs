use crate::{
    error::IndexResult,
    pipe::{
        iter::{Elements, Rows},
        map::MapPipe,
    },
    primitives::{point::Point, size::Size},
};

pub mod image;
pub mod iter;
pub mod kernel;
pub mod map;

pub trait Pipe {
    type Item;

    fn get(&self, point: Point) -> IndexResult<Self::Item>;
    fn size(&self) -> Size;
    fn rows(self) -> Rows<Self::Item, Self>
    where
        Self: Sized,
    {
        Rows::new(self)
    }
    fn elements(self) -> Elements<Self::Item, Self>
    where
        Self: Sized,
    {
        Elements::new(self)
    }
    fn map<T, F>(self, f: F) -> MapPipe<Self::Item, T, Self, F>
    where
        F: Fn(Self::Item) -> T,
        Self: Sized,
    {
        MapPipe::new(self, f)
    }
}

pub trait IntoPipe {
    type Item;
    type IntoPipe: Pipe<Item = Self::Item>;

    fn into_pipe(self) -> Self::IntoPipe;
}

impl<P: Pipe> IntoPipe for P {
    type Item = P::Item;
    type IntoPipe = P;

    fn into_pipe(self) -> Self::IntoPipe {
        self
    }
}

pub trait FromPipe<T>: Sized {
    fn from_pipe<P>(pipe: P) -> Self
    where
        P: IntoPipe<Item = T>;
}
