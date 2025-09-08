use crate::{component::kernel::Kernel, error::IndexResult, lens::Lens, prelude::{Margin, Point}};

#[derive(Default)]
pub struct IdentityKernel;

impl IdentityKernel {
    pub fn new() -> Self {
        IdentityKernel
    }
}

impl<T> Kernel<T, T> for IdentityKernel {
    fn apply<S>(&self, source: &S, point: Point) -> IndexResult<T>
    where
        S: Lens<Item = T> {
        source.look(point)
    }

    fn margin(&self) -> Margin {
        Margin::unified(0)
    }
}
