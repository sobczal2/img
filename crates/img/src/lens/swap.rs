use std::{cell::RefCell, rc::Rc};

use crate::{error::IndexResult, lens::Lens, primitive::{point::Point, size::Size}};

pub struct SwapLens<S> {
    source: Rc<RefCell<S>>,
}

impl<S> Clone for SwapLens<S> {
    fn clone(&self) -> Self {
        Self { source: self.source.clone() }
    }
}

impl<S> SwapLens<S>
    where S: Lens
{
    pub fn new(source: S) -> Self
    {
        Self { source: Rc::new(RefCell::new(source)) }
    }

    pub fn swap(&self, source: S) {
        self.source.replace_with(|_| source);
    }
}

impl<S> Lens for SwapLens<S>
    where S: Lens
{
    type Item = S::Item;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        self.source.borrow().get(point)
    }

    fn size(&self) -> Size {
        self.source.borrow().size()
    }
}
