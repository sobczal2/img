use std::marker::PhantomData;

use thiserror::Error;

use crate::{
    component::{
        kernel::Kernel,
        primitive::{
            Point,
            Size,
            SizeCreationError,
        },
    },
    error::{
        IndexError,
        IndexResult,
    },
    lens::Lens,
    prelude::{
        Area,
        Offset,
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
    working_area: Area,
    _phantom_data: PhantomData<T>,
}

impl<S, K, T> KernelLens<S, K, T>
where
    S: Lens,
    K: Kernel<S::Item, T>,
{
    pub fn new(source: S, kernel: K) -> Result<Self, CreationError> {
        let margin = kernel.margin();

        let size = source.size().shrink_by_margin(kernel.margin()).map_err(|e| match e {
            SizeCreationError::WidthZero => CreationError::KernelTooBigX,
            SizeCreationError::HeightZero => CreationError::KernelTooBigY,
            _ => unreachable!("unexpected error returned from shrink_by_margin"),
        })?;

        let top_left =
            Point::new(margin.left(), margin.top()).expect("unexpected error in Point::new");

        let working_area = Area::new(size, top_left);

        Ok(Self { source, kernel, working_area, _phantom_data: Default::default() })
    }
}

impl<S, K, T> Lens for KernelLens<S, K, T>
where
    S: Lens,
    K: Kernel<S::Item, T>,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        if !self.working_area.contains(&point) {
            return Err(IndexError::OutOfBounds);
        }
        let offset: Offset = self.working_area.top_left().into();
        let source_point = point.translate(offset).expect("unexpected error in Point::translate");

        self.kernel.apply(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.working_area.size()
    }
}
