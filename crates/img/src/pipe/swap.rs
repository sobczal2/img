use std::{cell::RefCell, rc::Rc};

use crate::{error::IndexResult, pipe::Pipe, primitive::{point::Point, size::Size}};

pub struct SwapPipe<S> {
    source: Rc<RefCell<S>>,
}

impl<S> Clone for SwapPipe<S> {
    fn clone(&self) -> Self {
        Self { source: self.source.clone() }
    }
}

impl<S> SwapPipe<S>
    where S: Pipe
{
    pub fn new(source: S) -> Self
    {
        Self { source: Rc::new(RefCell::new(source)) }
    }

    pub fn swap(&self, source: S) {
        self.source.replace_with(|_| source);
    }
}

impl<S> Pipe for SwapPipe<S>
    where S: Pipe
{
    type Item = S::Item;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        self.source.borrow().get(point)
    }

    fn size(&self) -> Size {
        self.source.borrow().size()
    }
}
