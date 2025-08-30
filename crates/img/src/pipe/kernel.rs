use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    component::kernel::Kernel,
    error::IndexResult,
    pipe::Pipe,
    primitive::{margin::Margin, point::Point, size::Size},
};

#[derive(Debug, Error)]
pub enum KernelPipeCreationError {
    #[error("kernel's width is too big")]
    KernelTooBigX,
    #[error("kernel's height is too big")]
    KernelTooBigY,
}

pub struct KernelPipe<P, K, T> {
    source: P,
    kernel: K,
    size: Size,
    margin: Margin,
    _phantom_data: PhantomData<T>,
}

impl<P: Pipe, K, T> KernelPipe<P, K, T>
where
    K: Kernel<P::Item, T>,
{
    pub fn new(source: P, kernel: K) -> Result<Self, KernelPipeCreationError> {
        if source.size().width() < kernel.size().width() {
            return Err(KernelPipeCreationError::KernelTooBigX);
        }

        if source.size().height() < kernel.size().height() {
            return Err(KernelPipeCreationError::KernelTooBigX);
        }

        let margin = Margin::from_size(kernel.size());

        let width = source.size().width() - margin.left() - margin.right();
        let height = source.size().height() - margin.top() - margin.bottom();

        // SAFETY: width, height are not zero after earlier checks
        let size = Size::from_usize(width, height).unwrap();

        Ok(Self {
            source,
            kernel,
            size,
            margin,
            _phantom_data: Default::default(),
        })
    }
}

impl<P: Pipe, K, T> Pipe for KernelPipe<P, K, T>
where
    K: Kernel<P::Item, T>,
{
    type Item = T;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        let source_point = Point::new(
            point.x() + self.margin.left(),
            point.y() + self.margin.top(),
        );

        self.kernel.apply(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.size
    }
}
