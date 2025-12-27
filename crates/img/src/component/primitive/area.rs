use thiserror::Error;

use super::{
    Margin,
    Offset,
    Point,
    Size,
    SizeCreationError,
};

#[derive(Debug, Error)]
pub enum AreaCreationError {
    #[error("resulting size invalid: {0}")]
    SizeInvalid(#[from] SizeCreationError),
}

pub type AreaCreationResult<T> = Result<T, AreaCreationError>;

/// Represents a 2D area defined by size and top left point.
#[derive(Debug, Clone, Copy)]
pub struct Area {
    size: Size,
    top_left: Point,
}

impl Area {
    /// Create an [`Area`] from specified size and top left point.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// // Create a 1x1 area in without any offset
    /// let without_offset = Area::new(Size::new(1, 1)?, Point::new(0, 0)?);
    ///
    /// // Create a 500x1000 area in with 100x50 offset
    /// let with_offset = Area::new(Size::new(500, 1000)?, Point::new(100, 50)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(size: Size, top_left: Point) -> Self {
        Self { size, top_left }
    }

    /// Create an [`Area`] from applying margin to some size.
    ///
    /// Returns [`Size`] if resulting size is valid `CreationError` otherwise.
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
    /// let with_offset = Area::from_cropped_size(Size::new(50, 50)?, Margin::new(20, 30, 20, 15)?)?;
    /// assert_eq!(with_offset.size(), Size::new(5, 10)?);
    /// assert_eq!(with_offset.top_left(), Point::new(15, 20)?);
    ///
    /// // Create a 5x10 area in without offset
    /// let without_offset = Area::from_cropped_size(Size::new(50, 50)?, Margin::new(0, 45, 40, 0)?)?;
    /// assert_eq!(without_offset.size(), Size::new(5, 10)?);
    /// assert_eq!(without_offset.top_left(), Point::new(0, 0)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_cropped_size(size: Size, margin: Margin) -> AreaCreationResult<Self> {
        let size = size.shrink_by_margin(margin)?;
        // SAFETY: margin.left() and margin.top() are guaranted to be less than DIMENSION_MAX.
        let top_left =
            Point::new(margin.left(), margin.top()).expect("unexpected error in Point::new");

        Ok(Self { size, top_left })
    }

    /// Get [`Area`]'s size.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Get [`Area`]'s top left point.
    pub fn top_left(&self) -> Point {
        self.top_left
    }

    /// Get [`Area`]'s top right point.
    ///
    /// Returns the point at the top right corner of the area (top_left.x + width - 1, top_left.y).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let area = Area::new(Size::new(10, 20)?, Point::new(100, 50)?);
    /// let top_right = area.top_right();
    ///
    /// assert_eq!(top_right, Point::new(109, 50)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn top_right(&self) -> Point {
        // SAFETY: translate returns Err only if resulting offset is negative,
        // here offset we apply has always positive values, so the resulting
        // offset will also always be positive. Also cast from usize to isize of
        // width is always safe since `width` <= `DIMENSION_MAX` < `isize::MAX`.
        self.top_left
            .translate(
                Offset::new(
                    (self.size.width() - 1)
                        .try_into()
                        .expect("unexpected error when converting usize to isize"),
                    0,
                )
                .expect("unexpected error in Offset::new"),
            )
            .expect("unexpected error in Point::translate")
    }

    /// Get [`Area`]'s bottom left point.
    ///
    /// Returns point at bottom left corner of area (top_left.x, top_left.y + height - 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let area = Area::new(Size::new(10, 20)?, Point::new(100, 50)?);
    /// let bottom_left = area.bottom_left();
    ///
    /// assert_eq!(bottom_left, Point::new(100, 69)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn bottom_left(&self) -> Point {
        // SAFETY: translate returns Err only if resulting offset is negative,
        // here offset we apply has always positive values, so the resulting
        // offset will also always be positive. Also cast from usize to isize of
        // height is always safe since `width` <= `DIMENSION_MAX` < `isize::MAX`.
        self.top_left
            .translate(
                Offset::new(
                    0,
                    (self.size.height() - 1)
                        .try_into()
                        .expect("unexpected error when converting usize to isize"),
                )
                .expect("unexpected error in Offset::new"),
            )
            .expect("unexpected error in Point::translate")
    }

    /// Get [`Area`]'s bottom right point.
    ///
    /// Returns the point at the bottom right corner of the area 
    /// (top_left.x + width - 1, top_left.y + height - 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let area = Area::new(Size::new(10, 20)?, Point::new(100, 50)?);
    /// let bottom_right = area.bottom_right();
    ///
    /// assert_eq!(bottom_right, Point::new(109, 69)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn bottom_right(&self) -> Point {
        // SAFETY: translate returns Err only if resulting offset is negative,
        // here offset we apply has always positive values, so the resulting
        // offset will also always be positive. Also cast from usize to isize of
        // width and height is always safe since `width` <= `DIMENSION_MAX` < `isize::MAX`.
        self.top_left
            .translate(
                Offset::new(
                    (self.size.width() - 1)
                        .try_into()
                        .expect("unexpected error when converting usize to isize"),
                    (self.size.height() - 1)
                        .try_into()
                        .expect("unexpected error when converting usize to isize"),
                )
                .expect("unexpected error in Offset::new"),
            )
            .expect("unexpected error in Point::translate")
    }

    /// Checks if [`Point`] is contained within [`Area`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// // Create a 1x1 area in without any offset
    /// let without_offset = Area::new(Size::new(1, 1)?, Point::new(0, 0)?);
    ///
    /// assert!(without_offset.contains(&Point::new(0, 0)?));
    /// assert!(!without_offset.contains(&Point::new(0, 1)?));
    /// assert!(!without_offset.contains(&Point::new(1, 0)?));
    /// assert!(!without_offset.contains(&Point::new(1, 1)?));
    ///
    /// // Create a 500x1000 area in with 100x50 offset
    /// let with_offset = Area::new(Size::new(500, 1000)?, Point::new(100, 50)?);
    ///
    /// assert!(!with_offset.contains(&Point::new(0, 0)?));
    /// assert!(!with_offset.contains(&Point::new(99, 50)?));
    /// assert!(!with_offset.contains(&Point::new(100, 49)?));
    /// assert!(with_offset.contains(&Point::new(100, 50)?));
    /// assert!(with_offset.contains(&Point::new(599, 1049)?));
    /// assert!(!with_offset.contains(&Point::new(600, 1049)?));
    /// assert!(!with_offset.contains(&Point::new(599, 1050)?));
    /// assert!(!with_offset.contains(&Point::new(600, 1050)?));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let size = Size::new(100, 200).unwrap();
        let top_left = Point::new(50, 75).unwrap();
        let area = Area::new(size, top_left);

        assert_eq!(area.size(), size);
        assert_eq!(area.top_left(), top_left);
    }

    #[test]
    fn test_from_cropped_size_ok() {
        let size = Size::new(50, 50).unwrap();
        let margin = Margin::new(20, 30, 20, 15).unwrap();
        let area = Area::from_cropped_size(size, margin).unwrap();

        assert_eq!(area.size(), Size::new(5, 10).unwrap());
        assert_eq!(area.top_left(), Point::new(15, 20).unwrap());

        // Test zero margins
        let zero_margin = Margin::new(0, 0, 0, 0).unwrap();
        let unchanged = Area::from_cropped_size(size, zero_margin).unwrap();
        assert_eq!(unchanged.size(), size);
        assert_eq!(unchanged.top_left(), Point::new(0, 0).unwrap());
    }

    #[test]
    fn test_from_cropped_size_err() {
        let size = Size::new(10, 20).unwrap();
        
        // Test invalid margin that would make width zero
        let margin_width_zero = Margin::new(0, 5, 0, 5).unwrap();
        assert!(Area::from_cropped_size(size, margin_width_zero).is_err());

        // Test invalid margin that would make height zero
        let margin_height_zero = Margin::new(10, 0, 10, 0).unwrap();
        assert!(Area::from_cropped_size(size, margin_height_zero).is_err());
    }

    #[test]
    fn test_size() {
        let size = Size::new(100, 200).unwrap();
        let area = Area::new(size, Point::new(50, 75).unwrap());
        assert_eq!(area.size(), size);
    }

    #[test]
    fn test_top_left() {
        let top_left = Point::new(100, 200).unwrap();
        let area = Area::new(Size::new(50, 75).unwrap(), top_left);
        assert_eq!(area.top_left(), top_left);
    }

    #[test]
    fn test_top_right() {
        let area = Area::new(Size::new(10, 20).unwrap(), Point::new(100, 50).unwrap());
        let top_right = area.top_right();
        
        // top_right should be (100 + 10 - 1, 50) = (109, 50)
        assert_eq!(top_right, Point::new(109, 50).unwrap());

        // Test 1x1 area
        let single = Area::new(Size::new(1, 1).unwrap(), Point::new(5, 10).unwrap());
        assert_eq!(single.top_right(), Point::new(5, 10).unwrap());
    }

    #[test]
    fn test_bottom_left() {
        let area = Area::new(Size::new(10, 20).unwrap(), Point::new(100, 50).unwrap());
        let bottom_left = area.bottom_left();
        
        // bottom_left should be (100, 50 + 20 - 1) = (100, 69)
        assert_eq!(bottom_left, Point::new(100, 69).unwrap());

        // Test 1x1 area
        let single = Area::new(Size::new(1, 1).unwrap(), Point::new(5, 10).unwrap());
        assert_eq!(single.bottom_left(), Point::new(5, 10).unwrap());
    }

    #[test]
    fn test_bottom_right() {
        let area = Area::new(Size::new(10, 20).unwrap(), Point::new(100, 50).unwrap());
        let bottom_right = area.bottom_right();
        
        // bottom_right should be (100 + 10 - 1, 50 + 20 - 1) = (109, 69)
        assert_eq!(bottom_right, Point::new(109, 69).unwrap());

        // Test 1x1 area
        let single = Area::new(Size::new(1, 1).unwrap(), Point::new(5, 10).unwrap());
        assert_eq!(single.bottom_right(), Point::new(5, 10).unwrap());
    }

    #[test]
    fn test_contains() {
        let area = Area::new(Size::new(500, 1000).unwrap(), Point::new(100, 50).unwrap());
        
        // Test corners (should be contained)
        assert!(area.contains(&Point::new(100, 50).unwrap())); // top_left
        assert!(area.contains(&Point::new(599, 50).unwrap())); // top_right
        assert!(area.contains(&Point::new(100, 1049).unwrap())); // bottom_left
        assert!(area.contains(&Point::new(599, 1049).unwrap())); // bottom_right
        
        // Test outside corners (should not be contained)
        assert!(!area.contains(&Point::new(99, 50).unwrap())); // left of area
        assert!(!area.contains(&Point::new(600, 50).unwrap())); // right of area
        assert!(!area.contains(&Point::new(100, 49).unwrap())); // above area
        assert!(!area.contains(&Point::new(100, 1050).unwrap())); // below area
        
        // Test 1x1 area
        let single = Area::new(Size::new(1, 1).unwrap(), Point::new(5, 10).unwrap());
        assert!(single.contains(&Point::new(5, 10).unwrap()));
        assert!(!single.contains(&Point::new(4, 10).unwrap()));
        assert!(!single.contains(&Point::new(6, 10).unwrap()));
        assert!(!single.contains(&Point::new(5, 9).unwrap()));
        assert!(!single.contains(&Point::new(5, 11).unwrap()));
    }
}
