use std::marker::PhantomData;

use crate::{error::IndexResult, primitives::{point::Point, size::Size}, view::View};

pub trait Kernel<S, T> {
    fn apply(&self, view: impl View<S>, point: Point) -> IndexResult<T>;
    fn size(&self) -> Size;
}

pub struct KernelView<S, T, V, K>
    where V: View<S>,
    K: Kernel<S, T>
{
    inner: V,
    kernel: K,
    size: Size,
    _phantom_s: PhantomData<S>,
    _phantom_t: PhantomData<T>,
}

impl <S, T, V, K> KernelView<S, T, V, K> {
}

impl<S, T, V, K> View<T> for KernelView<S, T, V, K>
    where V: View<S>,
    K: Kernel<S, T>
{
    fn get(&self, point: Point) -> IndexResult<T> {

    }

    fn size(&self) -> Size {
        self.size
    }
}
