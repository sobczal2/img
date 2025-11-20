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
pub enum KernelLensCreationError {
    #[error("kernel's width is too big")]
    KernelWidthTooBig,
    #[error("kernel's height is too big")]
    KernelHeightTooBig,
}

pub type KernelLensCreationResult<T> = std::result::Result<T, KernelLensCreationError>;

/// A [`Lens`] that applies [`Kernel`] onto `source`
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
    /// Create [`KernelLens`] with specified `source` and `kernel`.
    ///
    /// Returns [`KernelLens`] if `source`'s size is at least margin.left + margin.right + 1 in
    /// width and margin.top + margin.bottom + 1.
    pub fn new(source: S, kernel: K) -> Result<Self, KernelLensCreationError> {
        let margin = kernel.margin();

        let size = source.size().shrink_by_margin(kernel.margin()).map_err(|e| match e {
            SizeCreationError::WidthZero => KernelLensCreationError::KernelWidthTooBig,
            SizeCreationError::HeightZero => KernelLensCreationError::KernelHeightTooBig,
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

        self.kernel.evaluate(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.working_area.size()
    }
}
