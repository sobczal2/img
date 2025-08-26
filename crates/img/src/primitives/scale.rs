use thiserror::Error;

use crate::primitives::{point::Point, size::{Size, SizeCreationError}};



#[derive(Error, Debug)]
#[error("invalid scale")]
pub struct InvalidScale;

#[derive(Debug, Copy, Clone)]
pub struct Scale(f32, f32);

const MIN_SCALE: f32 = 1e-4;
const MAX_SCALE: f32 = 1e4;

impl Scale {
    pub fn new(x: f32, y: f32) -> Result<Self, InvalidScale> {
        if !(MIN_SCALE..MAX_SCALE).contains(&x) {
            return Err(InvalidScale);
        }

        if !(MIN_SCALE..MAX_SCALE).contains(&y) {
            return Err(InvalidScale);
        }

        Ok(Self(x, y))
    }

    pub fn apply(&self, size: Size) -> Result<Size, SizeCreationError> {
        let new_width: f32 = size.width() as f32 * self.0;
        let new_height: f32 = size.height() as f32 * self.1;

        Size::from_usize(new_width as usize, new_height as usize)
    }

    pub fn translate(&self, point: Point) -> Point {
        let new_x = point.x() as f32 / self.0;
        let new_y = point.y() as f32 / self.1;

        Point::new(new_x as usize, new_y as usize)
    }
}
