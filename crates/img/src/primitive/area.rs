use crate::primitive::{
    margin::Margin,
    point::Point,
    size::Size,
};

/// Represents a 2D area defined by size and top left point.
///
/// # Examples
///
/// ```
/// use img::primitive::{area::Area, size::Size, point::Point};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
/// // Create a 1x1 area in without any offset
/// let small_top_left = Area::new(Size::from_usize(1, 1)?, Point::new(0, 0));
///
/// # Ok(())
/// # }
/// ```
pub struct Area {
    size: Size,
    top_left: Point,
}

impl Area {
    pub fn new(size: Size, top_left: Point) -> Self {
        Self { size, top_left }
    }

    pub fn from_cropped_size(size: Size, margin: Margin) -> Self {
        let width = size.width() - margin.left() - margin.right();
        let height = size.height() - margin.top() - margin.bottom();

        let size = Size::from_usize(width, height).unwrap();
        let top_left = Point::new(margin.left(), margin.top());

        Self { size, top_left }
    }

    pub fn contains(&self, point: Point) -> bool {
        let offset = point - self.top_left;

        let relative = match offset.try_into() {
            Ok(point) => point,
            Err(_) => return false,
        };

        self.size.contains(relative)
    }
}
