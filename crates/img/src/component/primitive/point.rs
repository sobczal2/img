use std::{
    cmp::Ordering,
    ops::Sub,
};

use thiserror::Error;

use super::{
    Offset,
    Size,
};
use crate::error::{
    IndexResult,
    OutOfBoundsError,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CreationError {
    #[error("invalid x value")]
    InvalidX,
    #[error("invalid y value")]
    InvalidY,
}

pub type CreationResult<T> = Result<T, CreationError>;

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
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Create a new [`Point`] with both dimensions equal to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let zero1 = Point::new(0, 0);
    /// let zero2 = Point::zero();
    ///
    /// assert_eq!(zero1, zero2);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Creates [`Point`] given 1D index based on [`Size`] of 2D structure represented by 1D array.
    /// Performs bounds check.
    ///
    /// Returns [`Point`] if successful, [`OutOfBoundsError`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::from_usize(2, 2)?;
    ///
    /// assert_eq!(Point::from_index(0, size)?, Point::new(0, 0));
    /// assert_eq!(Point::from_index(1, size)?, Point::new(1, 0));
    /// assert_eq!(Point::from_index(2, size)?, Point::new(0, 1));
    /// assert_eq!(Point::from_index(3, size)?, Point::new(1, 1));
    ///
    /// assert!(Point::from_index(4, Size::from_usize(2, 2)?).is_err());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_index(index: usize, size: Size) -> IndexResult<Self> {
        let point = Point::new(index % size.width(), index / size.width());
        if !size.contains(&point) {
            return Err(OutOfBoundsError);
        }

        Ok(Point::new(index % size.width(), index / size.width()))
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
    /// Returns `usize` if successful, [`OutOfBoundsError`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let point_top_left = Point::new(0, 0);
    /// let point_top_right = Point::new(1, 0);
    /// let point_bottom_left = Point::new(0, 1);
    /// let point_bottom_right = Point::new(1, 1);
    ///
    /// let array = vec![0, 1, 2, 3];
    /// let size = Size::from_usize(2, 2)?;
    ///
    /// assert_eq!(array[point_top_left.index(size)?], 0);
    /// assert_eq!(array[point_top_right.index(size)?], 1);
    /// assert_eq!(array[point_bottom_left.index(size)?], 2);
    /// assert_eq!(array[point_bottom_right.index(size)?], 3);
    ///
    /// assert!(Point::new(2, 2).index(Size::from_usize(2, 2)?).is_err());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn index(&self, size: Size) -> IndexResult<usize> {
        if !size.contains(self) {
            return Err(OutOfBoundsError);
        }
        Ok(self.y * size.width().get() + self.x)
    }

    /// Translate [`Point`] by given [`Offset`].
    ///
    /// Returns [`Point`] if point components are non-negative, [`CreationError`] otherwise.
    ///
    /// # Errors
    ///
    /// * `CreationError::InvalidX` - if x is negative after applying offset.
    /// * `CreationError::InvalidY` - if y is negative after applying offset.
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
    /// let point = Point::new(100, 200);
    ///
    /// assert_eq!(point.translate(Offset::new(10, 20))?, Point::new(110, 220));
    /// assert_eq!(point.translate(Offset::new(-10, 20))?, Point::new(90, 220));
    /// assert_eq!(point.translate(Offset::new(10, -20))?, Point::new(110, 180));
    /// assert_eq!(point.translate(Offset::new(-10, -20))?, Point::new(90, 180));
    ///
    /// assert!(Point::new(10, 10).translate(Offset::new(-10, -10)).is_ok());
    /// assert_eq!(
    ///     Point::new(10, 10).translate(Offset::new(-11, -10)),
    ///     PointCreationResult::Err(PointCreationError::InvalidX)
    /// );
    /// assert_eq!(
    ///     Point::new(10, 10).translate(Offset::new(-10, -11)),
    ///     PointCreationResult::Err(PointCreationError::InvalidY)
    /// );
    /// assert_eq!(
    ///     Point::new(10, 10).translate(Offset::new(-11, -11)),
    ///     PointCreationResult::Err(PointCreationError::InvalidX)
    /// );
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate(mut self, offset: Offset) -> CreationResult<Self> {
        let x = self.x as isize + offset.x();
        let y = self.y as isize + offset.y();

        self.x = x.try_into().map_err(|_| CreationError::InvalidX)?;
        self.y = y.try_into().map_err(|_| CreationError::InvalidY)?;

        Ok(self)
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
    /// assert_eq!(Point::new(10, 20) - Point::new(5, 10), Offset::new(5, 10));
    /// assert_eq!(Point::new(10, 20) - Point::new(20, 10), Offset::new(-10, 10));
    /// assert_eq!(Point::new(10, 20) - Point::new(20, 40), Offset::new(-10, -20));
    ///
    /// # Ok(())
    /// # }
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x as isize - rhs.x as isize;
        let y = self.y as isize - rhs.y as isize;

        Offset::new(x, y)
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
    /// assert_eq!(Point::new(10, 10).partial_cmp(&Point::new(10, 10)), Some(Ordering::Equal));
    /// assert_eq!(Point::new(10, 10).partial_cmp(&Point::new(20, 20)), Some(Ordering::Less));
    /// assert_eq!(Point::new(10, 10).partial_cmp(&Point::new(10, 20)), Some(Ordering::Less));
    /// assert_eq!(Point::new(10, 10).partial_cmp(&Point::new(20, 10)), Some(Ordering::Less));
    /// assert_eq!(Point::new(20, 20).partial_cmp(&Point::new(10, 10)), Some(Ordering::Greater));
    /// assert_eq!(Point::new(20, 10).partial_cmp(&Point::new(10, 10)), Some(Ordering::Greater));
    /// assert_eq!(Point::new(10, 20).partial_cmp(&Point::new(10, 10)), Some(Ordering::Greater));
    /// assert_eq!(Point::new(20, 10).partial_cmp(&Point::new(10, 20)), None);
    /// assert_eq!(Point::new(10, 20).partial_cmp(&Point::new(20, 10)), None);
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
    type Error = CreationError;

    /// Create [`Point`] from [`Offset`].
    ///
    /// Returns [`Point`] if point components are non-negative, [`CreationError`] otherwise.
    ///
    /// # Errors
    ///
    /// * `CreationError::InvalidX` - if x is negative after applying offset.
    /// * `CreationError::InvalidY` - if y is negative after applying offset.
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
    /// assert_eq!(Point::try_from(Offset::new(1, 1))?, Point::new(1, 1));
    ///
    /// assert_eq!(
    ///     Point::try_from(Offset::new(-1, -1)),
    ///     PointCreationResult::Err(PointCreationError::InvalidX)
    /// );
    /// assert_eq!(
    ///     Point::try_from(Offset::new(1, -1)),
    ///     PointCreationResult::Err(PointCreationError::InvalidY)
    /// );
    /// assert_eq!(
    ///     Point::try_from(Offset::new(-1, 1)),
    ///     PointCreationResult::Err(PointCreationError::InvalidX)
    /// );
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: Offset) -> CreationResult<Self> {
        let x: usize = value.x().try_into().map_err(|_| CreationError::InvalidX)?;
        let y: usize = value.y().try_into().map_err(|_| CreationError::InvalidY)?;

        Ok(Point::new(x, y))
    }
}
