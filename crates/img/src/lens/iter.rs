use crate::{
    component::primitive::Point,
    error::{
        IndexError,
        IndexResult,
    },
    lens::Lens,
};

/// Iterator for going over [`RowElements`].
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

        // SAFETY: max value passed here is height - 1, which is guaranted to be less than
        // DIMENSION_MAX, since height is less than or equal to DIMENSION_MAX.
        let elements = RowElements::new(self.source, self.current - 1)
            .expect("unexpected error in RowElements::new");

        Some(elements)
    }
}

#[derive(Clone)]
pub struct RowElements<'a, S> {
    source: &'a S,
    current: Option<Point>,
}

impl<'a, S> RowElements<'a, S> {
    fn new(source: &'a S, row: usize) -> IndexResult<Self> {
        let point = Point::new(0, row).map_err(|_| IndexError::OutOfBounds)?;
        Ok(Self { source, current: Some(point) })
    }
}

impl<'a, S> Iterator for RowElements<'a, S>
where
    S: Lens,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(current) => {
                let value = self.source.look(current).expect("bug in lens implementation");
                if current.x() + 1 == self.source.size().width() {
                    self.current = None
                } else {
                    // SAFETY: x parameter in point::new is always less than width, which is less
                    // than or equal to DIMENSION_MAX
                    self.current = Some(
                        Point::new(current.x() + 1, current.y())
                            .expect("unexpected error in Point::new"),
                    );
                }

                Some(value)
            }
            None => None,
        }
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
