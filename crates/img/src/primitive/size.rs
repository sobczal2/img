use std::{cmp::Ordering, num::NonZeroUsize};

use thiserror::Error;

use crate::primitive::point::Point;

#[derive(Debug, Error)]
pub enum SizeCreationError {
    #[error("width is zero")]
    WidthZero,
    #[error("height is zero")]
    HeightZero,
}

/// Represents a 2D size. Minimum size is 1x1.
/// # Examples
/// ```
/// use img::primitive::size::Size;
///
/// // Create a smallest possible size
/// let size = Size::from_usize(1, 1).unwrap();
///
/// // Create a 100x100 size
/// let size100 = Size::from_usize(100, 100).unwrap();
///
/// // Tries to create a size, but width value is 0
/// let invalid_size = Size::from_usize(0, 10).unwrap_err();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(NonZeroUsize, NonZeroUsize);

impl Size {

    /// Create a new `Size` with the specified width and height.
    ///
    /// # Returns
    /// 
    /// Returns `Size` with specified width and height.
    ///
    /// # Examples
    /// ```
    /// use img::primitive::size::Size;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let small = Size::new(1.try_into()?, 2.try_into()?);
    /// let medium = Size::new(100.try_into()?, 200.try_into()?);
    /// let large = Size::new(100_000.try_into()?, 2_000_000.try_into()?);
    ///
    /// # Ok(())
    /// }
    /// ```
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self(width, height)
    }

    /// Create a new `Size` with the specified width and height. Unlike `new` this takes in `usize`
    /// arguments and can fail in case width or height is 0.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Size)` if both parameters valid, otherwise returns a `SizeCreationError`.
    ///
    /// # Errors
    ///
    /// * `SizeCreationError::WidthZero` - if `width` is 0.
    /// * `SizeCreationError::HeightZero` - if `height` is 0.
    ///
    /// # Examples
    /// ```
    /// use img::primitive::size::Size;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let small = Size::from_usize(1, 2)?;
    /// let medium = Size::from_usize(100, 200)?;
    /// let large = Size::from_usize(100_000, 2_000_000);
    ///
    /// // Invalid size
    /// assert!(Size::from_usize(0, 10).is_err());
    /// assert!(Size::from_usize(10, 0).is_err());
    /// assert!(Size::from_usize(0, 0).is_err());
    ///
    /// # Ok(())
    /// }
    /// ```
    pub fn from_usize(width: usize, height: usize) -> Result<Self, SizeCreationError> {
        let width: NonZeroUsize = width.try_into().map_err(|_| SizeCreationError::WidthZero)?;
        let height: NonZeroUsize = height
            .try_into()
            .map_err(|_| SizeCreationError::HeightZero)?;

        Ok(Size(width, height))
    }

    /// Create a new `Size` from specified radius. Radius is defined as distance between central
    /// point and any border.
    ///
    /// # Returns
    ///
    /// Returns `Size`, we always get a valid value since radius is non-negative.
    ///
    /// # Examples
    /// ```
    /// use img::primitive::size::Size;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let small = Size::from_radius(0);
    /// assert_eq!((small.width(), small.height()), (1, 1));
    ///
    /// let medium = Size::from_radius(15);
    /// assert_eq!((medium.width(), medium.height()), (31, 31));
    ///
    /// let large = Size::from_radius(10_000);
    /// assert_eq!((large.width(), large.height()), (20_001, 20_001));
    ///
    /// # Ok(())
    /// }
    /// ```
    pub fn from_radius(radius: usize) -> Self {
        let diameter = 2 * radius + 1;

        // SAFETY: diameter always positive
        Self::from_usize(diameter, diameter).unwrap()
    }

    pub fn width(&self) -> usize {
        self.0.into()
    }

    pub fn height(&self) -> usize {
        self.1.into()
    }

    pub fn area(&self) -> usize {
        self.width() * self.height()
    }

    pub fn center(&self) -> Point {
        Point::new(self.width() / 2, self.height() / 2)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x() < self.width() && point.y() < self.height()
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        if self.0 < other.0 && self.1 < other.1 {
            return Some(Ordering::Less);
        }

        if self.0 > other.0 && self.1 > other.1 {
            return Some(Ordering::Greater);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_basic() {
        assert!(Size::from_usize(1, 1).unwrap().contains(Point::new(0, 0)));
        assert!(!Size::from_usize(1, 1).unwrap().contains(Point::new(1, 0)));
        assert!(!Size::from_usize(1, 1).unwrap().contains(Point::new(0, 1)));
        assert!(!Size::from_usize(1, 1).unwrap().contains(Point::new(1, 1)));

        assert!(Size::from_usize(2, 2).unwrap().contains(Point::new(0, 0)));
        assert!(Size::from_usize(2, 2).unwrap().contains(Point::new(1, 0)));
        assert!(Size::from_usize(2, 2).unwrap().contains(Point::new(0, 1)));
        assert!(Size::from_usize(2, 2).unwrap().contains(Point::new(1, 1)));

        assert!(!Size::from_usize(2, 2).unwrap().contains(Point::new(2, 0)));
        assert!(!Size::from_usize(2, 2).unwrap().contains(Point::new(0, 2)));
        assert!(!Size::from_usize(2, 2).unwrap().contains(Point::new(2, 2)));
        assert!(!Size::from_usize(2, 2).unwrap().contains(Point::new(2, 1)));
        assert!(!Size::from_usize(2, 2).unwrap().contains(Point::new(1, 2)));

        assert!(Size::from_usize(1, 2).unwrap().contains(Point::new(0, 0)));
        assert!(!Size::from_usize(1, 2).unwrap().contains(Point::new(1, 0)));
        assert!(Size::from_usize(1, 2).unwrap().contains(Point::new(0, 1)));
        assert!(!Size::from_usize(1, 2).unwrap().contains(Point::new(1, 1)));

        assert!(Size::from_usize(2, 1).unwrap().contains(Point::new(0, 0)));
        assert!(Size::from_usize(2, 1).unwrap().contains(Point::new(1, 0)));
        assert!(!Size::from_usize(2, 1).unwrap().contains(Point::new(0, 1)));
        assert!(!Size::from_usize(2, 1).unwrap().contains(Point::new(1, 1)));
    }
}
