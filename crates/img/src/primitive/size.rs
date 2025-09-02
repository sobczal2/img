use std::{
    cmp::Ordering,
    num::NonZeroUsize,
};
use thiserror::Error;

use crate::primitive::{
    margin::Margin,
    point::Point,
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("width is zero")]
    WidthZero,
    #[error("height is zero")]
    HeightZero,
}

pub type CreationResult = Result<Size, CreationError>;

/// Represents a 2D size. Minimum size is 1x1.
///
/// # Examples
///
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
    /// Create a new `Size` with the specified `width` and `height`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::primitive::size::Size;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let small = Size::new(1.try_into()?, 2.try_into()?);
    /// let medium = Size::new(100.try_into()?, 200.try_into()?);
    /// let large = Size::new(100_000.try_into()?, 2_000_000.try_into()?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self(width, height)
    }

    /// Create a new `Size` with the specified width and height. Unlike `new` this takes in `usize`
    /// arguments and can fail in case width or height is 0.
    ///
    /// Returns `Ok(Size)` if both parameters valid, otherwise returns a `CreationError`.
    ///
    /// # Errors
    ///
    /// * `CreationError::WidthZero` - if `width` is 0.
    /// * `CreationError::HeightZero` - if `height` is 0.
    ///
    /// # Examples
    ///
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
    /// # }
    /// ```
    pub fn from_usize(width: usize, height: usize) -> CreationResult {
        let width: NonZeroUsize = width.try_into().map_err(|_| CreationError::WidthZero)?;
        let height: NonZeroUsize = height.try_into().map_err(|_| CreationError::HeightZero)?;

        Ok(Size(width, height))
    }

    /// Create a new `Size` from specified radius. Radius is defined as distance between central
    /// point and any border.
    ///
    /// # Examples
    ///
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
    /// # }
    /// ```
    pub fn from_radius(radius: usize) -> Self {
        let diameter = 2 * radius + 1;

        // SAFETY: diameter always positive
        Self::from_usize(diameter, diameter).unwrap()
    }

    /// Returns Size's width as `usize`. Is guaranteed to not be 0.
    pub fn width(&self) -> usize {
        self.0.into()
    }

    /// Returns Size's height as `usize`. Is guaranteed to not be 0.
    pub fn height(&self) -> usize {
        self.1.into()
    }

    /// Get area of the size (width*height).
    ///
    /// Returns Size's area as `usize`. Is guaranteed to not be 0.
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }

    /// Get rounded up middle point.
    ///
    /// Returns Size's middle point. Rounds point up in case dimension is even.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::primitive::{
    ///     point::Point,
    ///     size::Size,
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let smallest = Size::from_usize(1, 1)?;
    /// assert_eq!(smallest.middle(), Point::new(0, 0));
    ///
    /// let even = Size::from_usize(10, 10)?;
    /// assert_eq!(even.middle(), Point::new(5, 5));
    ///
    /// let odd = Size::from_usize(11, 11)?;
    /// assert_eq!(odd.middle(), Point::new(5, 5));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn middle(&self) -> Point {
        Point::new(self.width() / 2, self.height() / 2)
    }

    /// Checks if point is within `Size` bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::primitive::{
    ///     point::Point,
    ///     size::Size,
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let small = Size::from_usize(1, 1)?;
    /// assert!(small.contains(Point::new(0, 0)));
    /// assert!(!small.contains(Point::new(1, 0)));
    /// assert!(!small.contains(Point::new(0, 1)));
    ///
    /// let medium = Size::from_usize(15, 30)?;
    /// assert!(medium.contains(Point::new(0, 0)));
    /// assert!(medium.contains(Point::new(14, 29)));
    /// assert!(!medium.contains(Point::new(15, 0)));
    /// assert!(!medium.contains(Point::new(0, 30)));
    ///
    /// let large = Size::from_usize(1000, 1000)?;
    /// assert!(large.contains(Point::new(0, 0)));
    /// assert!(large.contains(Point::new(999, 999)));
    /// assert!(!large.contains(Point::new(1000, 0)));
    /// assert!(!large.contains(Point::new(0, 1000)));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains(&self, point: Point) -> bool {
        point.x() < self.width() && point.y() < self.height()
    }

    pub fn apply_margin(&self, margin: Margin) -> CreationResult {
        if margin.left() + margin.right() >= self.width() {
            return Err(CreationError::WidthZero);
        }

        if margin.top() + margin.bottom() >= self.height() {
            return Err(CreationError::HeightZero);
        }

        let width = self.width() - margin.left() - margin.right();
        let height = self.height() - margin.top() - margin.bottom();

        Ok(Size::from_usize(width, height).unwrap())
    }
}

impl PartialOrd for Size {
    /// Returns `Some(Ordering)` of sizes or `None` if it is not possible to compare them.
    ///
    /// A `Size` `a` is less than or equal to `b` if both `width` and `height` components
    /// are less than or equal. If one component is greater and other is smaller
    /// then it returns `None`.
    ///
    /// # Examples
    /// ```
    /// use img::primitive::size::Size;
    /// use std::cmp::Ordering;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(
    ///     Size::from_usize(10, 10)?.partial_cmp(&Size::from_usize(10, 10)?),
    ///     Some(Ordering::Equal)
    /// );
    /// assert_eq!(
    ///     Size::from_usize(10, 10)?.partial_cmp(&Size::from_usize(20, 20)?),
    ///     Some(Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Size::from_usize(10, 10)?.partial_cmp(&Size::from_usize(10, 20)?),
    ///     Some(Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Size::from_usize(10, 10)?.partial_cmp(&Size::from_usize(20, 10)?),
    ///     Some(Ordering::Less)
    /// );
    /// assert_eq!(
    ///     Size::from_usize(20, 20)?.partial_cmp(&Size::from_usize(10, 10)?),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Size::from_usize(20, 10)?.partial_cmp(&Size::from_usize(10, 10)?),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Size::from_usize(10, 20)?.partial_cmp(&Size::from_usize(10, 10)?),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(Size::from_usize(20, 10)?.partial_cmp(&Size::from_usize(10, 20)?), None);
    /// assert_eq!(Size::from_usize(10, 20)?.partial_cmp(&Size::from_usize(20, 10)?), None);
    /// # Ok(())
    /// # }
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        if self.0 <= other.0 && self.1 <= other.1 {
            return Some(Ordering::Less);
        }

        if self.0 >= other.0 && self.1 >= other.1 {
            return Some(Ordering::Greater);
        }

        None
    }
}
