use std::ops::Neg;

use super::Point;

/// Represents a 2D offset between 2 `Point`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    x: isize,
    y: isize,
}

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
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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
        Self::new(value.x() as isize, value.y() as isize)
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
