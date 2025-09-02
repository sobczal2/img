use crate::{
    lens::Lens,
    primitive::point::Point,
};

#[derive(Clone)]
pub struct Rows<'a, P> {
    lens: &'a P,
    current: usize,
}

impl<'a, P> Rows<'a, P> {
    pub fn new(lens: &'a P) -> Self {
        Self { lens, current: 0 }
    }
}

impl<'a, P> Iterator for Rows<'a, P>
where
    P: Lens,
{
    type Item = RowElements<'a, P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.lens.size().height() {
            return None;
        }

        self.current += 1;

        Some(RowElements::new(self.lens, self.current - 1))
    }
}

#[derive(Clone)]
pub struct RowElements<'a, P> {
    lens: &'a P,
    current: Point,
}

impl<'a, P> RowElements<'a, P> {
    fn new(lens: &'a P, row: usize) -> Self {
        Self { lens, current: Point::new(0, row) }
    }
}

impl<'a, P: Lens> Iterator for RowElements<'a, P>
where
    P: Lens,
{
    type Item = P::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x() == self.lens.size().width() {
            return None;
        }

        let value = self.lens.get(self.current).expect("bug in lens implementation");
        self.current = Point::new(self.current.x() + 1, self.current.y());

        Some(value)
    }
}

#[derive(Clone)]
pub struct Elements<'a, P> {
    lens: &'a P,
    current: usize,
}

impl<'a, P> Elements<'a, P> {
    pub fn new(lens: &'a P) -> Self {
        Self { lens, current: 0 }
    }
}

impl<'a, P: Lens> Iterator for Elements<'a, P>
where
    P: Lens,
{
    type Item = P::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let point = match Point::from_index(self.current, self.lens.size()) {
            Ok(point) => point,
            Err(_) => return None,
        };
        let value = self.lens.get(point).expect("bug in lens implementation");
        self.current += 1;

        Some(value)
    }
}
