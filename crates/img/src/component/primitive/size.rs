use std::cmp::Ordering;
use thiserror::Error;

use crate::image::DIMENSION_MAX;

use super::{
    Margin,
    Point,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CreationError {
    #[error("width is zero")]
    WidthZero,
    #[error("height is zero")]
    HeightZero,
    #[error("width too big")]
    WidthTooBig,
    #[error("height too big")]
    HeightTooBig,
}

pub type CreationResult<T> = Result<T, CreationError>;

/// Represents a 2D size. Minimum size is 1x1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    width: usize,
    height: usize,
}

impl Size {
    /// Create a new [`Size`] with specified `width` and `height`. This will fail
    /// in case `width` or `height` is 0 or larger than [`DIMENSION_MAX`].
    ///
    /// Returns [`Size`] if both parameters valid, otherwise returns a `CreationError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::new(100, 200).unwrap();
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(width: usize, height: usize) -> CreationResult<Self> {
        if width > DIMENSION_MAX {
            return Err(CreationError::WidthTooBig);
        }

        if width == 0 {
            return Err(CreationError::WidthZero);
        }

        if height > DIMENSION_MAX {
            return Err(CreationError::HeightTooBig);
        }

        if height == 0 {
            return Err(CreationError::HeightZero);
        }

        Ok(Self { width, height })
    }

    /// Create a new [`Size`] from specified radius. Radius is defined as distance between central
    /// point and any border.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let small = Size::from_radius(0)?;
    /// assert_eq!(small.width(), 1);
    /// assert_eq!(small.height(), 1);
    ///
    /// let medium = Size::from_radius(15)?;
    /// assert_eq!(medium.width(), 31);
    /// assert_eq!(medium.height(), 31);
    ///
    /// let large = Size::from_radius(10_000)?;
    /// assert_eq!(large.width(), 20_001);
    /// assert_eq!(large.height(), 20_001);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_radius(radius: usize) -> CreationResult<Self> {
        if radius > DIMENSION_MAX / 2 {
            return Err(CreationError::WidthTooBig)
        }

        let diameter = 2 * radius + 1;
        Self::new(diameter, diameter)
    }

    /// Get [`Size`]'s width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get [`Size`]'s height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Calculate [`Size`]'s area (width * height).
    ///
    /// Returns [`Size`]'s area as `usize`. Is guaranteed to not be 0.
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    /// Get rounded up middle point.
    ///
    /// Returns [`Size`]'s middle point. Rounds point up in case dimension is even.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let smallest = Size::new(1, 1)?;
    /// assert_eq!(smallest.middle(), Point::new(0, 0));
    ///
    /// let even = Size::new(10, 10)?;
    /// assert_eq!(even.middle(), Point::new(5, 5));
    ///
    /// let odd = Size::new(11, 11)?;
    /// assert_eq!(odd.middle(), Point::new(5, 5));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn middle(&self) -> Point {
        Point::new(self.width / 2, self.height / 2)
    }

    /// Checks if point is within [`Size`] bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let small = Size::new(1, 1)?;
    /// assert!(small.contains(&Point::new(0, 0)));
    /// assert!(!small.contains(&Point::new(1, 0)));
    /// assert!(!small.contains(&Point::new(0, 1)));
    ///
    /// let medium = Size::new(15, 30)?;
    /// assert!(medium.contains(&Point::new(0, 0)));
    /// assert!(medium.contains(&Point::new(14, 29)));
    /// assert!(!medium.contains(&Point::new(15, 0)));
    /// assert!(!medium.contains(&Point::new(0, 30)));
    ///
    /// let large = Size::new(1000, 1000)?;
    /// assert!(large.contains(&Point::new(0, 0)));
    /// assert!(large.contains(&Point::new(999, 999)));
    /// assert!(!large.contains(&Point::new(1000, 0)));
    /// assert!(!large.contains(&Point::new(0, 1000)));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains(&self, point: &Point) -> bool {
        point.x() < self.width && point.y() < self.height
    }

    /// Shrink [`Size`] by [`Margin`] - this results in a [`Size`] reduced by margins.
    ///
    /// Returns modifed [`Size`] or [`CreationError`] in case resulting [`Size`] would not be
    /// valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     component::primitive::{
    ///         SizeCreationError,
    ///         SizeCreationResult,
    ///     },
    ///     prelude::*,
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::new(10, 20)?;
    ///
    /// // Reduce size by 2 from the top, 3 from the right, 4 from the bottom, 5 from the left.
    /// let reduced_size = size.shrink_by_margin(Margin::new(2, 3, 4, 5)?)?;
    /// assert_eq!(reduced_size.width(), 2);
    /// assert_eq!(reduced_size.height(), 14);
    ///
    /// let invalid_width_size = size.shrink_by_margin(Margin::new(0, 4, 0, 6)?);
    /// assert_eq!(invalid_width_size, SizeCreationResult::Err(SizeCreationError::WidthZero));
    ///
    /// let invalid_height_size = size.shrink_by_margin(Margin::new(14, 0, 6, 0)?);
    /// assert_eq!(invalid_height_size, SizeCreationResult::Err(SizeCreationError::HeightZero));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn shrink_by_margin(&self, margin: Margin) -> CreationResult<Self> {
        if margin.left() + margin.right() >= self.width {
            return Err(CreationError::WidthZero);
        }

        if margin.top() + margin.bottom() >= self.height {
            return Err(CreationError::HeightZero);
        }

        let width = self.width - margin.left() - margin.right();
        let height = self.height - margin.top() - margin.bottom();

        Size::new(width, height)
    }

    /// Extend [`Size`] by [`Margin`] - this results in a [`Size`] increased by margins.
    ///
    /// Returns modifed [`Size`] or [`CreationError`] in case resulting [`Size`] would not be
    /// valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     component::primitive::{
    ///         SizeCreationError,
    ///         SizeCreationResult,
    ///     },
    ///     prelude::*,
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::new(10, 20)?;
    ///
    /// // Increase size by 2 from the top, 3 from the right, 4 from the bottom, 5 from the left.
    /// let increased_size = size.extend_by_margin(Margin::new(2, 3, 4, 5)?)?;
    ///
    /// assert_eq!(increased_size.width(), 18);
    /// assert_eq!(increased_size.height(), 26);
    ///
    /// let invalid_width_size = size.extend_by_margin(Margin::new(0, DIMENSION_MAX - 1, 0, 6)?);
    /// assert_eq!(invalid_width_size, SizeCreationResult::Err(SizeCreationError::WidthTooBig));
    ///
    /// let invalid_height_size = size.extend_by_margin(Margin::new(DIMENSION_MAX - 1, 0, 6, 0)?);
    /// assert_eq!(invalid_height_size, SizeCreationResult::Err(SizeCreationError::HeightTooBig));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn extend_by_margin(&self, margin: Margin) -> CreationResult<Self> {
        if margin.left() + margin.right() + self.width > DIMENSION_MAX {
            return Err(CreationError::WidthTooBig);
        }

        if margin.top() + margin.bottom() + self.height > DIMENSION_MAX {
            return Err(CreationError::HeightTooBig);
        }

        let width = self.width + margin.left() + margin.right();
        let height = self.height + margin.top() + margin.bottom();

        Size::new(width, height)
    }
}

impl PartialOrd for Size {
    /// Returns [`Ordering`] of sizes or [`None`] if it is not possible to compare them.
    ///
    /// A [`Size`] `a` is less than or equal to `b` if both `width` and `height` components
    /// are less than or equal. If one component is greater and other is smaller
    /// then it returns [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// use std::cmp::Ordering;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(Size::new(10, 10)?.partial_cmp(&Size::new(10, 10)?), Some(Ordering::Equal));
    /// assert_eq!(Size::new(10, 10)?.partial_cmp(&Size::new(20, 20)?), Some(Ordering::Less));
    /// assert_eq!(Size::new(10, 10)?.partial_cmp(&Size::new(10, 20)?), Some(Ordering::Less));
    /// assert_eq!(Size::new(10, 10)?.partial_cmp(&Size::new(20, 10)?), Some(Ordering::Less));
    /// assert_eq!(Size::new(20, 20)?.partial_cmp(&Size::new(10, 10)?), Some(Ordering::Greater));
    /// assert_eq!(Size::new(20, 10)?.partial_cmp(&Size::new(10, 10)?), Some(Ordering::Greater));
    /// assert_eq!(Size::new(10, 20)?.partial_cmp(&Size::new(10, 10)?), Some(Ordering::Greater));
    /// assert_eq!(Size::new(20, 10)?.partial_cmp(&Size::new(10, 20)?), None);
    /// assert_eq!(Size::new(10, 20)?.partial_cmp(&Size::new(20, 10)?), None);
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        if self.width <= other.width && self.height <= other.height {
            return Some(Ordering::Less);
        }

        if self.width >= other.width && self.height >= other.height {
            return Some(Ordering::Greater);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ok() {
        assert!(Size::new(10, 20).is_ok());
        assert!(Size::new(DIMENSION_MAX, DIMENSION_MAX).is_ok());
    }

    #[test]
    fn new_err() {
        assert_eq!(Size::new(0, 10).unwrap_err(), CreationError::WidthZero);
        assert_eq!(Size::new(10, 0).unwrap_err(), CreationError::HeightZero);
        assert_eq!(Size::new(DIMENSION_MAX + 1, 10).unwrap_err(), CreationError::WidthTooBig);
        assert_eq!(Size::new(10, DIMENSION_MAX + 1).unwrap_err(), CreationError::HeightTooBig);
    }

    #[test]
    fn from_radius_ok() {
        assert!(Size::from_radius(0).is_ok());
        assert!(Size::from_radius(DIMENSION_MAX / 2).is_ok());
    }
}
