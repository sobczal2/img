use crate::{
    error::IndexResult,
    pipe::{
        cloned::ClonedPipe,
        iter::{Elements, Rows},
        map::MapPipe,
        remap::RemapPipe,
    },
    primitive::{point::Point, size::Size},
};

pub mod cloned;
pub mod image;
pub mod iter;
pub mod kernel;
pub mod map;
pub mod remap;

pub trait Pipe {
    type Item;

    fn get(&self, point: Point) -> IndexResult<Self::Item>;

    fn size(&self) -> Size;

    fn rows(self) -> Rows<Self>
    where
        Self: Sized,
    {
        Rows::new(self)
    }

    fn elements(self) -> Elements<Self>
    where
        Self: Sized,
    {
        Elements::new(self)
    }

    fn map<T, F>(self, f: F) -> MapPipe<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Item) -> T,
    {
        MapPipe::new(self, f)
    }

    fn remap<T, F>(self, f: F, size: Size) -> RemapPipe<Self, F>
    where
        Self: Sized,
        F: Fn(&Self, Point) -> T,
    {
        RemapPipe::new(self, f, size)
    }

    fn cloned<'a>(self) -> ClonedPipe<Self>
    where
        Self: Sized,
        Self::Item: Clone + 'a,
    {
        ClonedPipe::new(self)
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
