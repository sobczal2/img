use crate::{
    error::{IndexResult, OutOfBoundsError},
    primitives::size::Size,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(usize, usize);

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn to_index(&self, size: Size) -> IndexResult<usize> {
        if !size.contains(*self) {
            return Err(OutOfBoundsError);
        }
        Ok(unsafe { self.to_index_unchecked(size) })
    }

    /// .
    ///
    /// # Safety
    /// The caller has to guarantee that point is within size to get
    /// a valid index
    /// .
    pub unsafe fn to_index_unchecked(&self, size: Size) -> usize {
        self.y() * size.width() + self.x()
    }

    pub fn from_index(index: usize, size: Size) -> IndexResult<Self> {
        let point = Point::new(index % size.width(), index / size.width());
        if !size.contains(point) {
            return Err(OutOfBoundsError);
        }

        Ok(point)
    }

    /// .
    ///
    /// # Safety
    /// The caller has to guarantee that point is within size to get
    /// a valid index
    /// .
    pub unsafe fn from_index_unchecked(index: usize, size: Size) -> Self {
        Point::new(index % size.width(), index / size.width())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_idx_basic() {
        assert_eq!(
            Point::new(0, 0).to_index(Size::from_usize(1, 1).unwrap()),
            Ok(0)
        );
        assert_eq!(
            Point::new(0, 0).to_index(Size::from_usize(100, 100).unwrap()),
            Ok(0)
        );
        assert_eq!(
            Point::new(10, 0).to_index(Size::from_usize(100, 100).unwrap()),
            Ok(10)
        );
        assert_eq!(
            Point::new(1, 0).to_index(Size::from_usize(10, 10).unwrap()),
            Ok(1)
        );
        assert_eq!(
            Point::new(0, 1).to_index(Size::from_usize(10, 10).unwrap()),
            Ok(10)
        );
        assert_eq!(
            Point::new(2, 1).to_index(Size::from_usize(1, 1).unwrap()),
            Err(OutOfBoundsError)
        );
    }
}
