use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    component::{
        kernel::Kernel,
        primitive::{
            Margin,
            Point,
            Size,
        },
    },
    error::IndexResult,
    lens::Lens,
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
        let margin = kernel.margin();

        if source.size().width() < margin.left() + margin.right() + 1 {
            return Err(CreationError::KernelTooBigX);
        }

        if source.size().height() < margin.top() + margin.bottom() + 1 {
            return Err(CreationError::KernelTooBigY);
        }

        let size = source.size().apply_margin(kernel.margin()).unwrap();

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
