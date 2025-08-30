use thiserror::Error;

use crate::{
    error::IndexResult,
    pipe::Pipe,
    primitive::{point::Point, size::Size},
};

pub trait Kernel {
    type Out;
    type In;

    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<Self::Out>
    where
        P: Pipe<Item = Self::In>;

    fn size(&self) -> Size;
}

#[derive(Debug, Error)]
pub enum KernelPipeCreationError {
    #[error("kernel's width is too big")]
    KernelTooBigX,
    #[error("kernel's height is too big")]
    KernelTooBigY,
}

pub struct KernelPipe<P, K> {
    source: P,
    kernel: K,
    size: Size,
}

impl<P: Pipe, K: Kernel> KernelPipe<P, K> {
    pub fn new(source: P, kernel: K) -> Result<Self, KernelPipeCreationError> {
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
        })
    }
}

impl<P, K: Kernel> Pipe for KernelPipe<P, K>
where
    P: Pipe<Item = K::In>,
{
    type Item = K::Out;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        let margin_x = self.kernel.size().width() / 2;
        let margin_y = self.kernel.size().height() / 2;

        let source_point = Point::new(point.x() + margin_x, point.y() + margin_y);

        self.kernel.apply(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.size
    }
}
