use std::ops::Neg;

use thiserror::Error;

use crate::image::DIMENSION_MAX;

use super::Point;

/// Represents a 2D offset between 2 `Point`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    x: isize,
    y: isize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OffsetCreationError {
    #[error("x too big")]
    XTooBig,
    #[error("y too big")]
    YTooBig,
    #[error("x too small")]
    XTooSmall,
    #[error("y too small")]
    YTooSmall,
}

pub type OffsetCreationResult<T> = std::result::Result<T, OffsetCreationError>;

impl Offset {
    /// Create a new [`Offset`] with specified `x` and `y`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let offset = Offset::new(100, 200);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(x: isize, y: isize) -> OffsetCreationResult<Self> {
        if x >= DIMENSION_MAX as isize {
            return Err(OffsetCreationError::XTooBig);
        }
        if x <= -(DIMENSION_MAX as isize) {
            return Err(OffsetCreationError::XTooSmall);
        }
        if y >= DIMENSION_MAX as isize {
            return Err(OffsetCreationError::YTooBig);
        }
        if y <= -(DIMENSION_MAX as isize) {
            return Err(OffsetCreationError::YTooSmall);
        }

        Ok(Self { x, y })
    }

    /// Returns [`Offset`]'s x.
    pub fn x(&self) -> isize {
        self.x
    }

    /// Returns [`Offset`]'s y.
    pub fn y(&self) -> isize {
        self.y
    }
}

impl From<Point> for Offset {
    fn from(value: Point) -> Self {
        // SAFETY: point's x and y are less than DIMENSION_MAX
        Self::new(value.x() as isize, value.y() as isize).expect("unexpected error in Offset::new")
    }
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ok() {
        assert!(Offset::new(0, 0).is_ok());
        assert!(Offset::new(-(DIMENSION_MAX as isize) + 1, 0).is_ok());
        assert!(Offset::new(0, -(DIMENSION_MAX as isize) + 1).is_ok());
        assert!(Offset::new((DIMENSION_MAX as isize) - 1, 0).is_ok());
        assert!(Offset::new(0, (DIMENSION_MAX as isize) - 1).is_ok());
    }

    #[test]
    fn test_new_err() {
        assert_eq!(Offset::new(-(DIMENSION_MAX as isize), 0).unwrap_err(), OffsetCreationError::XTooSmall);
        assert_eq!(Offset::new(0, -(DIMENSION_MAX as isize)).unwrap_err(), OffsetCreationError::YTooSmall);
        assert_eq!(Offset::new(DIMENSION_MAX as isize, 0).unwrap_err(), OffsetCreationError::XTooBig);
        assert_eq!(Offset::new(0, DIMENSION_MAX as isize).unwrap_err(), OffsetCreationError::YTooBig);
    }

    #[test]
    fn test_from_point() {
        assert_eq!(Offset::from(Point::new(0, 0).unwrap()), Offset::new(0, 0).unwrap());
        assert_eq!(Offset::from(Point::new(DIMENSION_MAX - 1, 0).unwrap()), Offset::new((DIMENSION_MAX as isize) - 1, 0).unwrap());
        assert_eq!(Offset::from(Point::new(0, DIMENSION_MAX - 1).unwrap()), Offset::new(0, (DIMENSION_MAX as isize) - 1).unwrap());
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Offset::new(0,0).unwrap(), Offset::new(0,0).unwrap());
        assert_eq!(-Offset::new((DIMENSION_MAX as isize) - 1, 0).unwrap(), Offset::new(-(DIMENSION_MAX as isize) + 1, 0).unwrap());
        assert_eq!(-Offset::new(0, (DIMENSION_MAX as isize) - 1).unwrap(), Offset::new(0, -(DIMENSION_MAX as isize) + 1).unwrap());
    }
}
