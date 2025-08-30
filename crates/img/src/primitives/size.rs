use std::{cmp::Ordering, num::NonZeroUsize};

use thiserror::Error;

use crate::primitives::point::Point;

#[derive(Debug, Error)]
pub enum SizeCreationError {
    #[error("width is zero")]
    WidthZero,
    #[error("height is zero")]
    HeightZero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(NonZeroUsize, NonZeroUsize);

impl Size {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self(width, height)
    }

    pub fn from_usize(width: usize, height: usize) -> Result<Self, SizeCreationError> {
        let width: NonZeroUsize = width.try_into().map_err(|_| SizeCreationError::WidthZero)?;
        let height: NonZeroUsize = height
            .try_into()
            .map_err(|_| SizeCreationError::WidthZero)?;

        Ok(Size(width, height))
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
