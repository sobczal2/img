use crate::primitive::{margin::Margin, point::Point, size::Size};

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
