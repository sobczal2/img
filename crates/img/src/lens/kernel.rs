use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    component::kernel::Kernel,
    error::IndexResult,
    lens::Lens,
    primitive::{
        margin::Margin,
        point::Point,
        size::Size,
    },
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("kernel's width is too big")]
    KernelTooBigX,
    #[error("kernel's height is too big")]
    KernelTooBigY,
}

#[derive(Clone)]
pub struct KernelLens<S, K, T> {
    source: S,
    kernel: K,
    size: Size,
    margin: Margin,
    _phantom_data: PhantomData<T>,
}

impl<S, K, T> KernelLens<S, K, T>
where
    S: Lens,
    K: Kernel<S::Item, T>,
{
    pub fn new(source: S, kernel: K) -> Result<Self, CreationError> {
        if source.size().width() < kernel.size().width() {
            return Err(CreationError::KernelTooBigX);
        }

        if source.size().height() < kernel.size().height() {
            return Err(CreationError::KernelTooBigY);
        }

        let margin = Margin::from_size(kernel.size());

        let size = source.size().apply_margin(margin).unwrap();

        Ok(Self { source, kernel, size, margin, _phantom_data: Default::default() })
    }
}

impl<S, K, T> Lens for KernelLens<S, K, T>
where
    S: Lens,
    K: Kernel<S::Item, T>,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        let source_point =
            Point::new(point.x() + self.margin.left(), point.y() + self.margin.top());

        self.kernel.apply(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.size
    }
}
