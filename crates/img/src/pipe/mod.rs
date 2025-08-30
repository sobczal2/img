use crate::{
    component::kernel::Kernel,
    error::IndexResult,
    pipe::{
        cloned::ClonedPipe,
        iter::{Elements, Rows},
        kernel::{KernelPipe, KernelPipeCreationError},
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

    fn rows(&self) -> Rows<'_, Self>
    where
        Self: Sized,
    {
        Rows::new(self)
    }

    fn elements(&self) -> Elements<'_, Self>
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

    fn kernel<K, T>(self, kernel: K) -> Result<KernelPipe<Self, K, T>, KernelPipeCreationError>
    where
        Self: Sized,
        K: Kernel<Self::Item, T>,
    {
        KernelPipe::new(self, kernel)
    }
}

pub trait FromPipe<T>: Sized {
    fn from_pipe<P>(pipe: P) -> Self
    where
        P: Pipe<Item = T>;
}

#[cfg(feature = "parallel")]
pub trait FromPipePar<T>: Sized {
    fn from_pipe_par<P>(pipe: P) -> Self
    where
        P: Pipe<Item = T> + Send + Sync,
        P::Item: Send;
}
