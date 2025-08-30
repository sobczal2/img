use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    error::IndexResult,
    pipe::Pipe,
    primitives::{point::Point, size::Size},
};

pub trait Kernel<S, T> {
    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<T>
    where
        P: Pipe<Item = S>;
    fn size(&self) -> Size;
}

#[derive(Debug, Error)]
pub enum KernelPipeCreationError {
    #[error("kernel's width is too big")]
    KernelTooBigX,
    #[error("kernel's height is too big")]
    KernelTooBigY,
}

pub struct KernelPipe<S, T, V, K>
where
    V: Pipe<Item = S>,
    K: Kernel<S, T>,
{
    source: V,
    kernel: K,
    size: Size,
    _phantom_s: PhantomData<S>,
    _phantom_t: PhantomData<T>,
}

impl<S, T, V, K> KernelPipe<S, T, V, K>
where
    V: Pipe<Item = S>,
    K: Kernel<S, T>,
{
    pub fn new(source: V, kernel: K) -> Result<Self, KernelPipeCreationError> {
        if source.size().width() < kernel.size().width() {
            return Err(KernelPipeCreationError::KernelTooBigX);
        }

        if source.size().height() < kernel.size().height() {
            return Err(KernelPipeCreationError::KernelTooBigX);
        }

        let margin_x = kernel.size().width() / 2;
        let margin_y = kernel.size().height() / 2;

        let width = source.size().width() - 2 * margin_x;
        let height = source.size().height() - 2 * margin_y;

        // SAFETY: width, height are not zero after earlier checks
        let size = Size::from_usize(width, height).unwrap();

        Ok(Self {
            source,
            kernel,
            size,
            _phantom_s: Default::default(),
            _phantom_t: Default::default(),
        })
    }
}

impl<S, T, V, K> Pipe for KernelPipe<S, T, V, K>
where
    V: Pipe<Item = S>,
    K: Kernel<S, T>,
{
    type Item = T;

    fn get(&self, point: Point) -> IndexResult<T> {
        let margin_x = self.kernel.size().width() / 2;
        let margin_y = self.kernel.size().height() / 2;

        let source_point = Point::new(point.x() + margin_x, point.y() + margin_y);

        self.kernel.apply(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.size
    }
}
