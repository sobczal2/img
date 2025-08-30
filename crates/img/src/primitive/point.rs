use std::ops::Sub;

use thiserror::Error;

use crate::{
    error::{IndexResult, OutOfBoundsError},
    primitive::{offset::Offset, size::Size},
};

#[derive(Debug, Error)]
pub enum PointCreationError {
    #[error("invalid x value")]
    InvalidX,
    #[error("invalid y value")]
    InvalidY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(usize, usize);

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn zero() -> Self {
        Self(0, 0)
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

    pub fn offset_by(&self, offset: Offset) -> Result<Point, PointCreationError> {
        let x = self.0 as isize + offset.x();
        let y = self.1 as isize + offset.y();

        let x: usize = x.try_into().map_err(|_| PointCreationError::InvalidX)?;
        let y: usize = y.try_into().map_err(|_| PointCreationError::InvalidY)?;

        Ok(Point(x, y))
    }
}

impl Sub for Point {
    type Output = Offset;

    ///
    /// # Examples
    /// ```
    /// use img::primitive::{point::Point, offset::Offset};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(Point::new(10, 20) - Point::new(5, 10), Offset::new(5, 10));
    /// assert_eq!(Point::new(10, 20) - Point::new(20, 10), Offset::new(-10, 10));
    /// assert_eq!(Point::new(10, 20) - Point::new(20, 40), Offset::new(-10, -20));
    /// # Ok(())
    /// # }
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0 as isize - rhs.0 as isize;
        let y = self.1 as isize - rhs.1 as isize;

        Offset::new(x, y)
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
