use std::marker::PhantomData;

use crate::{error::IndexResult, primitives::{point::Point, size::Size}};

pub mod image;
pub mod kernel;

pub trait View<T> {
    fn get(&self, point: Point) -> IndexResult<T>;
    fn size(&self) -> Size;
}

pub trait AsView<'a, V, T>
where V: View<T>
{
    fn as_view(&'a self) -> V;
}

// TODO: create 2d iterators structure similar to Rows and Pixels.
// if performance is the same then replace the implementations in image.rs
pub struct ElementsIterator<'a, V, T>
where V: View<T>
{
    view: &'a V,
    current: Point,
    _phantom: PhantomData<T>,
}

impl<'a, V, T> ElementsIterator<'a, V, T>
where V: View<T>
{
    pub fn new(view: &'a V) -> Self {
        ElementsIterator { view, current: Point::zero(),_phantom: Default::default() }
    }
}

impl<'a, V, T> Iterator for ElementsIterator<'a, V, T>
    where V: View<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.view.size().contains(self.current) {
            return None;
        }

        let result = self.view.get(self.current).expect("bug in view implementation");
        if self.current.x() + 1 == self.view.size().width() {
            self.current = Point::new(0, self.current.y() + 1);
        }
        else {
            self.current = Point::new(self.current.x() + 1, self.current.y());
        }

        Some(result)
    }
}
