use std::rc::Rc;

use crate::{pipe::Pipe, primitives::point::Point};

pub struct Rows<T, P>
where
    P: Pipe<Item = T>,
{
    pipe: Rc<P>,
    current: usize,
}

impl<T, P> Rows<T, P>
where
    P: Pipe<Item = T>,
{
    pub fn new(pipe: P) -> Self {
        Self {
            pipe: Rc::new(pipe),
            current: 0,
        }
    }
}

impl<T, P> Iterator for Rows<T, P>
where
    P: Pipe<Item = T>,
{
    type Item = RowElements<T, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.pipe.size().height() {
            return None;
        }

        self.current += 1;

        Some(RowElements::new(self.pipe.clone(), self.current - 1))
    }
}

pub struct RowElements<T, P>
where
    P: Pipe<Item = T>,
{
    pipe: Rc<P>,
    current: Point,
}

impl<T, P> RowElements<T, P>
where
    P: Pipe<Item = T>,
{
    fn new(pipe: Rc<P>, row: usize) -> Self {
        Self {
            pipe,
            current: Point::new(0, row),
        }
    }
}

impl<T, P> Iterator for RowElements<T, P>
where
    P: Pipe<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x() == self.pipe.size().width() {
            return None;
        }

        let value = self
            .pipe
            .get(self.current)
            .expect("bug in pipe implementation");
        self.current = Point::new(self.current.x() + 1, self.current.y());

        Some(value)
    }
}

pub struct Elements<T, P>
where
    P: Pipe<Item = T>,
{
    pipe: P,
    current: usize,
}

impl<T, P> Elements<T, P>
where
    P: Pipe<Item = T>,
{
    pub fn new(pipe: P) -> Self {
        Self { pipe, current: 0 }
    }
}

impl<T, P> Iterator for Elements<T, P>
where
    P: Pipe<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let point = match Point::from_index(self.current, self.pipe.size()) {
            Ok(point) => point,
            Err(_) => return None,
        };
        let value = self.pipe.get(point).expect("bug in pipe implementation");
        self.current += 1;

        Some(value)
    }
}
