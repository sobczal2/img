use crate::{
    lens::Lens,
    primitive::point::Point,
};

#[derive(Clone)]
pub struct Rows<'a, S> {
    source: &'a S,
    current: usize,
}

impl<'a, S> Rows<'a, S> {
    pub fn new(source: &'a S) -> Self {
        Self { source, current: 0 }
    }
}

impl<'a, S> Iterator for Rows<'a, S>
where
    S: Lens,
{
    type Item = RowElements<'a, S>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.source.size().height() {
            return None;
        }

        self.current += 1;

        Some(RowElements::new(self.source, self.current - 1))
    }
}

#[derive(Clone)]
pub struct RowElements<'a, S> {
    source: &'a S,
    current: Point,
}

impl<'a, S> RowElements<'a, S> {
    fn new(source: &'a S, row: usize) -> Self {
        Self { source, current: Point::new(0, row) }
    }
}

impl<'a, S> Iterator for RowElements<'a, S>
where
    S: Lens,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x() == self.source.size().width() {
            return None;
        }

        let value = self.source.look(self.current).expect("bug in lens implementation");
        self.current = Point::new(self.current.x() + 1, self.current.y());

        Some(value)
    }
}

#[derive(Clone)]
pub struct Elements<'a, S> {
    lens: &'a S,
    current: usize,
}

impl<'a, S> Elements<'a, S> {
    pub fn new(lens: &'a S) -> Self {
        Self { lens, current: 0 }
    }
}

impl<'a, S> Iterator for Elements<'a, S>
where
    S: Lens,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let point = match Point::from_index(self.current, self.lens.size()) {
            Ok(point) => point,
            Err(_) => return None,
        };
        let value = self.lens.look(point).expect("bug in lens implementation");
        self.current += 1;

        Some(value)
    }
}
