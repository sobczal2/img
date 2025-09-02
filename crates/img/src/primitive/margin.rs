use crate::primitive::{
    point::Point,
    size::Size,
};
use paste::paste;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Margin {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

macro_rules! define_swizzle {
    ($vertical:ident, $horizontal:ident) => {
        paste! {
            pub fn [<$vertical _ $horizontal>](&self) -> Point {
                Point::new(self.$horizontal, self.$vertical)
            }
        }
    };
}

impl Margin {
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        Margin { top, right, bottom, left }
    }

    pub fn unified(value: usize) -> Self {
        Margin { top: value, right: value, bottom: value, left: value }
    }

    pub fn from_size(size: Size) -> Self {
        let center = size.middle();
        let left = center.x();
        let top = center.y();
        let right = size.width() - center.x();
        let bottom = size.height() - center.y();
        Self::new(top, right, bottom, left)
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn right(&self) -> usize {
        self.right
    }

    pub fn bottom(&self) -> usize {
        self.bottom
    }

    pub fn left(&self) -> usize {
        self.left
    }

    define_swizzle!(top, left);
    define_swizzle!(bottom, left);
    define_swizzle!(top, right);
    define_swizzle!(bottom, right);
}
