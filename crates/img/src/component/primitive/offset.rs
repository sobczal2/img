use std::ops::Neg;

use thiserror::Error;

use crate::image::DIMENSION_MAX;

use super::Point;

/// Represents a 2D offset between 2 `Point`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    x: isize,
    y: isize,
}

#[derive(Debug, Error)]
pub enum OffsetCreationError {
    #[error("x too big")]
    XTooBig,
    #[error("y too big")]
    YTooBig,
    #[error("x too small")]
    XTooSmall,
    #[error("y too small")]
    YTooSmall,
}

pub type OffsetCreationResult<T> = std::result::Result<T, OffsetCreationError>;

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
    pub fn new(x: isize, y: isize) -> OffsetCreationResult<Self> {
        if x >= DIMENSION_MAX as isize {
            return Err(OffsetCreationError::XTooBig);
        }
        if x <= -(DIMENSION_MAX as isize) {
            return Err(OffsetCreationError::XTooSmall);
        }
        if y >= DIMENSION_MAX as isize {
            return Err(OffsetCreationError::YTooBig);
        }
        if y <= -(DIMENSION_MAX as isize) {
            return Err(OffsetCreationError::YTooSmall);
        }

        Ok(Self { x, y })
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
        // SAFETY: point's x and y are less than DIMENSION_MAX
        Self::new(value.x() as isize, value.y() as isize).expect("unexpected error in Offset::new")
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
