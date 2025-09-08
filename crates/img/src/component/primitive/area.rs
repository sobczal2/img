use thiserror::Error;

use super::{
    Margin,
    Offset,
    Point,
    Size,
    SizeCreationError,
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("resulting size invalid: {0}")]
    SizeInvalid(SizeCreationError),
}

pub type CreationResult<T> = Result<T, CreationError>;

/// Represents a 2D area defined by size and top left point.
#[derive(Debug, Clone, Copy)]
pub struct Area {
    size: Size,
    top_left: Point,
}

impl Area {
    /// Create an `Area` from specified size and top left point.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// // Create a 1x1 area in without any offset
    /// let without_offset = Area::new(Size::from_usize(1, 1)?, Point::new(0, 0));
    ///
    /// // Create a 500x1000 area in with 100x50 offset
    /// let with_offset = Area::new(Size::from_usize(500, 1000)?, Point::new(100, 50));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(size: Size, top_left: Point) -> Self {
        Self { size, top_left }
    }

    /// Create an `Area` from applying margin to some size.
    ///
    /// Returns `Size` if resulting size is valid `CreationError` otherwise.
    ///
    /// # Errors
    ///
    /// * `CreationError::InvalidSize` - if resulting size is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// // Create a 5x10 area in with 15x20 offset
    /// let with_offset =
    ///     Area::from_cropped_size(Size::from_usize(50, 50)?, Margin::new(20, 30, 20, 15))?;
    /// assert_eq!(with_offset.size(), Size::from_usize(5, 10)?);
    /// assert_eq!(with_offset.top_left(), Point::new(15, 20));
    ///
    /// // Create a 5x10 area in without offset
    /// let without_offset =
    ///     Area::from_cropped_size(Size::from_usize(50, 50)?, Margin::new(0, 45, 40, 0))?;
    /// assert_eq!(without_offset.size(), Size::from_usize(5, 10)?);
    /// assert_eq!(without_offset.top_left(), Point::new(0, 0));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_cropped_size(size: Size, margin: Margin) -> CreationResult<Self> {
        let width = size.width() - margin.left() - margin.right();
        let height = size.height() - margin.top() - margin.bottom();

        let size = Size::from_usize(width, height).unwrap();
        let top_left = Point::new(margin.left(), margin.top());

        Ok(Self { size, top_left })
    }

    /// Returns `Area`'s size.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns `Area`'s top left point
    pub fn top_left(&self) -> Point {
        self.top_left
    }

    /// Returns `Area`'s top left point
    pub fn top_right(&self) -> Point {
        self.top_left.translate(Offset::new(self.size.width() as isize, 0)).unwrap()
    }

    /// Returns `Area`'s top left point
    pub fn bottom_left(&self) -> Point {
        self.top_left.translate(Offset::new(0, self.size.height() as isize)).unwrap()
    }

    /// Returns `Area`'s top left point
    pub fn bottom_right(&self) -> Point {
        self.top_left
            .translate(Offset::new(self.size.width() as isize, self.size.height() as isize))
            .unwrap()
    }

    /// Checks if `Point` is contained within `Area`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// // Create a 1x1 area in without any offset
    /// let without_offset = Area::new(Size::from_usize(1, 1)?, Point::new(0, 0));
    ///
    /// assert!(without_offset.contains(&Point::new(0, 0)));
    /// assert!(!without_offset.contains(&Point::new(0, 1)));
    /// assert!(!without_offset.contains(&Point::new(1, 0)));
    /// assert!(!without_offset.contains(&Point::new(1, 1)));
    ///
    /// // Create a 500x1000 area in with 100x50 offset
    /// let with_offset = Area::new(Size::from_usize(500, 1000)?, Point::new(100, 50));
    ///
    /// assert!(!with_offset.contains(&Point::new(0, 0)));
    /// assert!(!with_offset.contains(&Point::new(99, 50)));
    /// assert!(!with_offset.contains(&Point::new(100, 49)));
    /// assert!(with_offset.contains(&Point::new(100, 50)));
    /// assert!(with_offset.contains(&Point::new(599, 1049)));
    /// assert!(!with_offset.contains(&Point::new(600, 1049)));
    /// assert!(!with_offset.contains(&Point::new(599, 1050)));
    /// assert!(!with_offset.contains(&Point::new(600, 1050)));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains(&self, point: &Point) -> bool {
        let offset = *point - self.top_left;

        let relative = match offset.try_into() {
            Ok(point) => point,
            Err(_) => return false,
        };

        self.size.contains(&relative)
    }
}
