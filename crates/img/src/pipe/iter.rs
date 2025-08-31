use crate::{
    pipe::Pipe,
    primitive::point::Point,
};

#[derive(Clone)]
pub struct Rows<'a, P> {
    pipe: &'a P,
    current: usize,
}

impl<'a, P> Rows<'a, P> {
    pub fn new(pipe: &'a P) -> Self {
        Self { pipe, current: 0 }
    }
}

impl<'a, P> Iterator for Rows<'a, P>
where
    P: Pipe,
{
    type Item = RowElements<'a, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.pipe.size().height() {
            return None;
        }

        self.current += 1;

        Some(RowElements::new(self.pipe, self.current - 1))
    }
}

#[derive(Clone)]
pub struct RowElements<'a, P> {
    pipe: &'a P,
    current: Point,
}

impl<'a, P> RowElements<'a, P> {
    fn new(pipe: &'a P, row: usize) -> Self {
        Self { pipe, current: Point::new(0, row) }
    }
}

impl<'a, P: Pipe> Iterator for RowElements<'a, P>
where
    P: Pipe,
{
    type Item = P::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x() == self.pipe.size().width() {
            return None;
        }

        let value = self.pipe.get(self.current).expect("bug in pipe implementation");
        self.current = Point::new(self.current.x() + 1, self.current.y());

        Some(value)
    }
}

#[derive(Clone)]
pub struct Elements<'a, P> {
    pipe: &'a P,
    current: usize,
}

impl<'a, P> Elements<'a, P> {
    pub fn new(pipe: &'a P) -> Self {
        Self { pipe, current: 0 }
    }
}

impl<'a, P: Pipe> Iterator for Elements<'a, P>
where
    P: Pipe,
{
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
