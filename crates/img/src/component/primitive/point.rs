use std::{
    cmp::Ordering,
    ops::Sub,
};

use thiserror::Error;

use super::{
    Offset,
    Size,
};
use crate::{
    error::{
        IndexError,
        IndexResult,
    },
    image::DIMENSION_MAX,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PointCreationError {
    #[error("x too big")]
    XTooBig,
    #[error("y too big")]
    YTooBig,
    #[error("x negative")]
    XNegative,
    #[error("y negative")]
    YNegative,
}

pub type PointCreationResult<T> = std::result::Result<T, PointCreationError>;

/// Represents point on a 2D structure. Both dimensions are represented as positive integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    /// Create a new [`Point`] with the specified `x` and `y`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let point = Point::new(100, 200);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(x: usize, y: usize) -> PointCreationResult<Self> {
        if x >= DIMENSION_MAX {
            return Err(PointCreationError::XTooBig);
        }

        if y >= DIMENSION_MAX {
            return Err(PointCreationError::YTooBig);
        }

        Ok(Self { x, y })
    }

    /// Create a new [`Point`] with both dimensions equal to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let zero1 = Point::new(0, 0)?;
    /// let zero2 = Point::zero();
    ///
    /// assert_eq!(zero1, zero2);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn zero() -> Self {
        Self::new(0, 0).expect("unexpected error in Point::new")
    }

    /// Creates [`Point`] given 1D index based on [`Size`] of 2D structure represented by 1D array.
    /// Performs bounds check.
    ///
    /// Returns [`Point`] if successful, [`IndexError`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::new(2, 2)?;
    ///
    /// assert_eq!(Point::from_index(0, size)?, Point::new(0, 0)?);
    /// assert_eq!(Point::from_index(1, size)?, Point::new(1, 0)?);
    /// assert_eq!(Point::from_index(2, size)?, Point::new(0, 1)?);
    /// assert_eq!(Point::from_index(3, size)?, Point::new(1, 1)?);
    ///
    /// assert!(Point::from_index(4, Size::new(2, 2)?).is_err());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_index(index: usize, size: Size) -> IndexResult<Self> {
        let point = Point::new(index % size.width(), index / size.width())
            .map_err(|_| IndexError::OutOfBounds)?;
        if !size.contains(&point) {
            return Err(IndexError::OutOfBounds);
        }

        Ok(point)
    }

    /// Returns [`Point`]'s x.
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns [`Point`]'s y.
    pub fn y(&self) -> usize {
        self.y
    }

    /// Create 1D index for [`Point`] based on [`Size`] of 2D structure represented by 1D array.
    /// Performs bounds check.
    ///
    /// Returns `usize` if successful, [`IndexError`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let point_top_left = Point::new(0, 0)?;
    /// let point_top_right = Point::new(1, 0)?;
    /// let point_bottom_left = Point::new(0, 1)?;
    /// let point_bottom_right = Point::new(1, 1)?;
    ///
    /// let array = vec![0, 1, 2, 3];
    /// let size = Size::new(2, 2)?;
    ///
    /// assert_eq!(array[point_top_left.index(size)?], 0);
    /// assert_eq!(array[point_top_right.index(size)?], 1);
    /// assert_eq!(array[point_bottom_left.index(size)?], 2);
    /// assert_eq!(array[point_bottom_right.index(size)?], 3);
    ///
    /// assert!(Point::new(2, 2)?.index(Size::new(2, 2)?).is_err());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn index(&self, size: Size) -> IndexResult<usize> {
        if !size.contains(self) {
            return Err(IndexError::OutOfBounds);
        }

        Ok(self.y * size.width() + self.x)
    }

    /// Translate [`Point`] by given [`Offset`].
    ///
    /// Returns [`Point`] if point components are non-negative, [`PointCreationError`] otherwise.
    ///
    /// # Errors
    ///
    /// * `PointCreationError::InvalidX` - if x is negative after applying offset.
    /// * `PointCreationError::InvalidY` - if y is negative after applying offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     component::primitive::{
    ///         PointCreationError,
    ///         PointCreationResult,
    ///     },
    ///     prelude::*,
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let point = Point::new(100, 200)?;
    ///
    /// assert_eq!(point.translate(Offset::new(10, 20)?)?, Point::new(110, 220)?);
    /// assert_eq!(point.translate(Offset::new(-10, 20)?)?, Point::new(90, 220)?);
    /// assert_eq!(point.translate(Offset::new(10, -20)?)?, Point::new(110, 180)?);
    /// assert_eq!(point.translate(Offset::new(-10, -20)?)?, Point::new(90, 180)?);
    ///
    /// assert!(Point::new(10, 10)?.translate(Offset::new(-10, -10)?).is_ok());
    /// assert_eq!(
    ///     Point::new(10, 10)?.translate(Offset::new(-11, -10)?).unwrap_err(),
    ///     PointCreationError::XNegative
    /// );
    /// assert_eq!(
    ///     Point::new(10, 10)?.translate(Offset::new(-10, -11)?).unwrap_err(),
    ///     PointCreationError::YNegative
    /// );
    /// assert_eq!(
    ///     Point::new(10, 10)?.translate(Offset::new(-11, -11)?).unwrap_err(),
    ///     PointCreationError::XNegative
    /// );
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate(self, offset: Offset) -> PointCreationResult<Self> {
        let x = self.x as isize + offset.x();
        let y = self.y as isize + offset.y();

        let new_x = x.try_into().map_err(|_| PointCreationError::XNegative)?;
        let new_y = y.try_into().map_err(|_| PointCreationError::YNegative)?;

        Self::new(new_x, new_y)
    }
}

impl Sub for Point {
    type Output = Offset;

    /// Subtract one [`Point`] from another
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// assert_eq!(Point::new(10, 20)? - Point::new(5, 10)?, Offset::new(5, 10)?);
    /// assert_eq!(Point::new(10, 20)? - Point::new(20, 10)?, Offset::new(-10, 10)?);
    /// assert_eq!(Point::new(10, 20)? - Point::new(20, 40)?, Offset::new(-10, -20)?);
    ///
    /// # Ok(())
    /// # }
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x as isize - rhs.x as isize;
        let y = self.y as isize - rhs.y as isize;

        // SAFETY: subtracting point from another is guaranteed to produce |value| < DIMENSION_MAX
        Offset::new(x, y).expect("unexpected error in Offset::new")
    }
}

impl PartialOrd for Point {
    /// Returns [`Ordering`] of [`Point`]s or none if it is not possible to compare them.
    ///
    /// A size `a` is less than or equal to `b` if both `width` and `height` components
    /// are less than or equal. If one component is greater and other is smaller
    /// then it returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// use std::cmp::Ordering;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// assert_eq!(Point::new(10, 10)?.partial_cmp(&Point::new(10, 10)?), Some(Ordering::Equal));
    /// assert_eq!(Point::new(10, 10)?.partial_cmp(&Point::new(20, 20)?), Some(Ordering::Less));
    /// assert_eq!(Point::new(10, 10)?.partial_cmp(&Point::new(10, 20)?), Some(Ordering::Less));
    /// assert_eq!(Point::new(10, 10)?.partial_cmp(&Point::new(20, 10)?), Some(Ordering::Less));
    /// assert_eq!(Point::new(20, 20)?.partial_cmp(&Point::new(10, 10)?), Some(Ordering::Greater));
    /// assert_eq!(Point::new(20, 10)?.partial_cmp(&Point::new(10, 10)?), Some(Ordering::Greater));
    /// assert_eq!(Point::new(10, 20)?.partial_cmp(&Point::new(10, 10)?), Some(Ordering::Greater));
    /// assert_eq!(Point::new(20, 10)?.partial_cmp(&Point::new(10, 20)?), None);
    /// assert_eq!(Point::new(10, 20)?.partial_cmp(&Point::new(20, 10)?), None);
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        if self.x <= other.x && self.y <= other.y {
            return Some(Ordering::Less);
        }

        if self.x >= other.x && self.y >= other.y {
            return Some(Ordering::Greater);
        }

        None
    }
}

impl TryFrom<Offset> for Point {
    type Error = PointCreationError;

    /// Create [`Point`] from [`Offset`].
    ///
    /// Returns [`Point`] if point components are non-negative, [`PointCreationError`] otherwise.
    ///
    /// # Errors
    ///
    /// * `PointCreationError::XNegative` - if x is negative after applying offset.
    /// * `PointCreationError::YNegative` - if y is negative after applying offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     component::primitive::{
    ///         PointCreationError,
    ///         PointCreationResult,
    ///     },
    ///     prelude::*,
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// assert_eq!(Point::try_from(Offset::new(1, 1)?)?, Point::new(1, 1)?);
    ///
    /// assert_eq!(Point::try_from(Offset::new(-1, -1)?).unwrap_err(), PointCreationError::XNegative);
    /// assert_eq!(Point::try_from(Offset::new(1, -1)?).unwrap_err(), PointCreationError::YNegative);
    /// assert_eq!(Point::try_from(Offset::new(-1, 1)?).unwrap_err(), PointCreationError::XNegative);
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: Offset) -> PointCreationResult<Self> {
        let x: usize = value.x().try_into().map_err(|_| PointCreationError::XNegative)?;
        let y: usize = value.y().try_into().map_err(|_| PointCreationError::YNegative)?;

        Point::new(x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_ok() {
        assert!(Point::new(0, 0).is_ok());
        assert!(Point::new(DIMENSION_MAX - 1, 0).is_ok());
        assert!(Point::new(0, DIMENSION_MAX - 1).is_ok());
    }

    #[test]
    fn test_new_err() {
        assert_eq!(Point::new(DIMENSION_MAX, 0).unwrap_err(), PointCreationError::XTooBig);
        assert_eq!(Point::new(0, DIMENSION_MAX).unwrap_err(), PointCreationError::YTooBig);
    }

    #[test]
    fn test_from_index_ok() {
        let size = Size::new(DIMENSION_MAX, DIMENSION_MAX).unwrap();
        assert_eq!(Point::from_index(1, size).unwrap(), Point::new(1, 0).unwrap());
        assert_eq!(Point::from_index(DIMENSION_MAX + 1, size).unwrap(), Point::new(1, 1).unwrap());
        assert_eq!(
            Point::from_index(DIMENSION_MAX * DIMENSION_MAX - 1, size).unwrap(),
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap()
        );
    }

    #[test]
    fn test_from_index_err() {
        assert_eq!(
            Point::from_index(
                DIMENSION_MAX * DIMENSION_MAX,
                Size::new(DIMENSION_MAX, DIMENSION_MAX).unwrap()
            )
            .unwrap_err(),
            IndexError::OutOfBounds
        );
        assert_eq!(
            Point::from_index(4, Size::new(2, 2).unwrap()).unwrap_err(),
            IndexError::OutOfBounds
        );
    }

    #[test]
    fn test_index_ok() {
        assert_eq!(Point::new(0, 0).unwrap().index(Size::new(1, 1).unwrap()).unwrap(), 0);
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, 0)
                .unwrap()
                .index(Size::new(DIMENSION_MAX, 1).unwrap())
                .unwrap(),
            DIMENSION_MAX - 1
        );
        assert_eq!(
            Point::new(0, DIMENSION_MAX - 1)
                .unwrap()
                .index(Size::new(1, DIMENSION_MAX).unwrap())
                .unwrap(),
            DIMENSION_MAX - 1
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .index(Size::new(DIMENSION_MAX, DIMENSION_MAX).unwrap())
                .unwrap(),
            DIMENSION_MAX * DIMENSION_MAX - 1
        );
    }

    #[test]
    fn test_index_err() {
        assert_eq!(
            Point::new(1, 0).unwrap().index(Size::new(1, 1).unwrap()).unwrap_err(),
            IndexError::OutOfBounds
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, 1)
                .unwrap()
                .index(Size::new(DIMENSION_MAX, 1).unwrap())
                .unwrap_err(),
            IndexError::OutOfBounds
        );
        assert_eq!(
            Point::new(1, DIMENSION_MAX - 1)
                .unwrap()
                .index(Size::new(1, DIMENSION_MAX).unwrap())
                .unwrap_err(),
            IndexError::OutOfBounds
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .index(Size::new(DIMENSION_MAX, DIMENSION_MAX - 1).unwrap())
                .unwrap_err(),
            IndexError::OutOfBounds
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .index(Size::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap())
                .unwrap_err(),
            IndexError::OutOfBounds
        );
    }

    #[test]
    fn test_translate_ok() {
        assert!(Point::new(0, 0).unwrap().translate(Offset::new(0, 0).unwrap()).is_ok());
        assert!(
            Point::new(0, 0)
                .unwrap()
                .translate(
                    Offset::new((DIMENSION_MAX as isize) - 1, (DIMENSION_MAX as isize) - 1)
                        .unwrap()
                )
                .is_ok()
        );
        assert!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .translate(Offset::new(0, 0).unwrap())
                .is_ok()
        );
        assert!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .translate(
                    Offset::new(-(DIMENSION_MAX as isize) + 1, -(DIMENSION_MAX as isize) + 1)
                        .unwrap()
                )
                .is_ok()
        );
    }

    #[test]
    fn test_translate_err() {
        assert_eq!(
            Point::new(1, 1)
                .unwrap()
                .translate(
                    Offset::new((DIMENSION_MAX as isize) - 1, (DIMENSION_MAX as isize) - 1)
                        .unwrap()
                )
                .unwrap_err(),
            PointCreationError::XTooBig
        );
        assert_eq!(
            Point::new(1, 1)
                .unwrap()
                .translate(Offset::new((DIMENSION_MAX as isize) - 1, 0).unwrap())
                .unwrap_err(),
            PointCreationError::XTooBig
        );
        assert_eq!(
            Point::new(1, 1)
                .unwrap()
                .translate(Offset::new(0, (DIMENSION_MAX as isize) - 1).unwrap())
                .unwrap_err(),
            PointCreationError::YTooBig
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .translate(Offset::new(1, 1).unwrap())
                .unwrap_err(),
            PointCreationError::XTooBig
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .translate(Offset::new(1, 0).unwrap())
                .unwrap_err(),
            PointCreationError::XTooBig
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .translate(Offset::new(0, 1).unwrap())
                .unwrap_err(),
            PointCreationError::YTooBig
        );
        assert_eq!(
            Point::new(0, 0).unwrap().translate(Offset::new(-1, -1).unwrap()).unwrap_err(),
            PointCreationError::XNegative
        );
        assert_eq!(
            Point::new(0, 0).unwrap().translate(Offset::new(-1, 1).unwrap()).unwrap_err(),
            PointCreationError::XNegative
        );
        assert_eq!(
            Point::new(0, 0).unwrap().translate(Offset::new(1, -1).unwrap()).unwrap_err(),
            PointCreationError::YNegative
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Point::new(0, 0).unwrap() - Point::new(0, 0).unwrap(),
            Offset::new(0, 0).unwrap()
        );
        assert_eq!(
            Point::new(0, 0).unwrap() - Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap(),
            Offset::new(-(DIMENSION_MAX as isize) + 1, -(DIMENSION_MAX as isize) + 1).unwrap()
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap()
                - Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap(),
            Offset::new(0, 0).unwrap()
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap() - Point::new(0, 0).unwrap(),
            Offset::new(DIMENSION_MAX as isize - 1, DIMENSION_MAX as isize - 1).unwrap()
        );
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(
            Point::new(0, 0).unwrap().partial_cmp(&Point::new(0, 0).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, 0)
                .unwrap()
                .partial_cmp(&Point::new(DIMENSION_MAX - 1, 0).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Point::new(0, DIMENSION_MAX - 1)
                .unwrap()
                .partial_cmp(&Point::new(0, DIMENSION_MAX - 1).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .partial_cmp(&Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap()),
            Some(Ordering::Equal)
        );

        assert_eq!(
            Point::new(DIMENSION_MAX - 1, 0).unwrap().partial_cmp(&Point::new(0, 0).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Point::new(0, DIMENSION_MAX - 1).unwrap().partial_cmp(&Point::new(0, 0).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .partial_cmp(&Point::new(0, 0).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .partial_cmp(&Point::new(DIMENSION_MAX - 1, 0).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1)
                .unwrap()
                .partial_cmp(&Point::new(0, DIMENSION_MAX - 1).unwrap()),
            Some(Ordering::Greater)
        );

        assert_eq!(
            Point::new(0, 0).unwrap().partial_cmp(&Point::new(DIMENSION_MAX - 1, 0).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Point::new(0, 0).unwrap().partial_cmp(&Point::new(0, DIMENSION_MAX - 1).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Point::new(0, 0)
                .unwrap()
                .partial_cmp(&Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Point::new(DIMENSION_MAX - 1, 0)
                .unwrap()
                .partial_cmp(&Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Point::new(0, DIMENSION_MAX - 1)
                .unwrap()
                .partial_cmp(&Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap()),
            Some(Ordering::Less)
        );

        assert_eq!(
            Point::new(DIMENSION_MAX - 1, 0)
                .unwrap()
                .partial_cmp(&Point::new(0, DIMENSION_MAX - 1).unwrap()),
            None
        );
        assert_eq!(
            Point::new(0, DIMENSION_MAX - 1)
                .unwrap()
                .partial_cmp(&Point::new(DIMENSION_MAX - 1, 0).unwrap()),
            None
        );
    }

    #[test]
    fn test_try_from_offset_ok() {
        assert_eq!(Point::try_from(Offset::new(0, 0).unwrap()).unwrap(), Point::new(0, 0).unwrap());
        assert_eq!(
            Point::try_from(Offset::new(DIMENSION_MAX as isize - 1, 0).unwrap()).unwrap(),
            Point::new(DIMENSION_MAX - 1, 0).unwrap()
        );
        assert_eq!(
            Point::try_from(Offset::new(0, DIMENSION_MAX as isize - 1).unwrap()).unwrap(),
            Point::new(0, DIMENSION_MAX - 1).unwrap()
        );
        assert_eq!(
            Point::try_from(
                Offset::new(DIMENSION_MAX as isize - 1, DIMENSION_MAX as isize - 1).unwrap()
            )
            .unwrap(),
            Point::new(DIMENSION_MAX - 1, DIMENSION_MAX - 1).unwrap()
        );
    }

    #[test]
    fn test_try_from_offset_err() {
        assert_eq!(
            Point::try_from(Offset::new(-(DIMENSION_MAX as isize) + 1, 0).unwrap()).unwrap_err(),
            PointCreationError::XNegative
        );
        assert_eq!(
            Point::try_from(Offset::new(0, -(DIMENSION_MAX as isize) + 1).unwrap()).unwrap_err(),
            PointCreationError::YNegative
        );
        assert_eq!(
            Point::try_from(
                Offset::new(-(DIMENSION_MAX as isize) + 1, -(DIMENSION_MAX as isize) + 1).unwrap()
            )
            .unwrap_err(),
            PointCreationError::XNegative
        );
    }
}
