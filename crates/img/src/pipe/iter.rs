use std::rc::Rc;

use crate::{pipe::Pipe, primitive::point::Point};

pub struct Rows<P> {
    pipe: Rc<P>,
    current: usize,
}

impl<P> Rows<P> {
    pub fn new(pipe: P) -> Self {
        Self {
            pipe: Rc::new(pipe),
            current: 0,
        }
    }
}

impl<P: Pipe> Iterator for Rows<P> {
    type Item = RowElements<P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.pipe.size().height() {
            return None;
        }

        self.current += 1;

        Some(RowElements::new(self.pipe.clone(), self.current - 1))
    }
}

pub struct RowElements<P> {
    pipe: Rc<P>,
    current: Point,
}

impl<P> RowElements<P> {
    fn new(pipe: Rc<P>, row: usize) -> Self {
        Self {
            pipe,
            current: Point::new(0, row),
        }
    }
}

impl<P: Pipe> Iterator for RowElements<P> {
    type Item = P::Item;

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

pub struct Elements<P> {
    pipe: P,
    current: usize,
}

impl<P> Elements<P> {
    pub fn new(pipe: P) -> Self {
        Self { pipe, current: 0 }
    }
}

impl<P: Pipe> Iterator for Elements<P> {
    type Item = P::Item;

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
