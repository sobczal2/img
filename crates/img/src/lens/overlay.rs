use thiserror::Error;

use crate::{
    component::primitive::{
        Area,
        Offset,
        Point,
        Size,
    },
    error::IndexResult,
    lens::Lens,
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OverlayLensCreationError {
    #[error("overlay_start out of bounds")]
    OverlayStartOutOfBounds,
    #[error("overlay is too big")]
    OverlayTooBig,
}

pub type OverlayLensCreationResult<T> = std::result::Result<T, OverlayLensCreationError>;

/// A [`Lens`] that returns values from `overlay` in `overlay_area` and `base` elsewhere.
pub struct OverlayLens<S1, S2> {
    base: S1,
    overlay: S2,
    overlay_area: Area,
}

impl<S1, S2> OverlayLens<S1, S2>
where
    S1: Lens,
    S2: Lens,
{

    /// Create [`OverlayLens`] with `base`, `overlay` and `overlay_area` with top-left point at
    /// `overlay_start` and size of `overlay`.
    ///
    /// Returns [`OverlayLens`] if `overlay` fits in `base`, [`OverlayLensCreationError`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     lens::Lens,
    ///     lens::overlay::OverlayLens,
    ///     lens::value::ValueLens,
    ///     prelude::*,
    /// };
    /// use std::iter::from_fn;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let base = ValueLens::new(Pixel::zero(), Size::new(2, 2)?);
    /// let overlay = ValueLens::new(Pixel::new([255, 0, 0, 255]), Size::new(1, 1)?);
    /// let overlay_start = Point::new(1, 1)?;
    ///
    /// let lens = OverlayLens::new(base, overlay, overlay_start)?;
    ///
    /// assert_eq!(lens.look(Point::new(0, 0)?)?.r(), 0);
    /// assert_eq!(lens.look(Point::new(1, 0)?)?.r(), 0);
    /// assert_eq!(lens.look(Point::new(0, 1)?)?.r(), 0);
    /// assert_eq!(lens.look(Point::new(1, 1)?)?.r(), 255);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base: S1, overlay: S2, overlay_start: Point) -> OverlayLensCreationResult<Self> {
        if !base.size().contains(&overlay_start) {
            return Err(OverlayLensCreationError::OverlayStartOutOfBounds);
        }

        let overlay_size = overlay.size();

        let bottom_right = Point::new(
            overlay_size.width() + overlay_start.x() - 1,
            overlay_size.height() + overlay_start.y() - 1,
        )
        .map_err(|_| OverlayLensCreationError::OverlayTooBig)?;

        if !base.size().contains(&bottom_right) {
            return Err(OverlayLensCreationError::OverlayTooBig);
        }

        Ok(Self { base, overlay, overlay_area: Area::new(overlay_size, overlay_start) })
    }
}

impl<S1, S2, T> Lens for OverlayLens<S1, S2>
where
    S1: Lens<Item = T>,
    S2: Lens<Item = T>,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        if self.overlay_area.contains(&point) {
            let offset = Offset::from(self.overlay_area.top_left());
            // SAFETY: since we checked point is in overlay area, then we are sure this translate
            // will return a valid point.
            return self.overlay.look(point.translate(-offset).expect("Unexpected error in Point::translate"));
        }

        self.base.look(point)
    }

    fn size(&self) -> Size {
        self.base.size()
    }
}

#[cfg(test)]
mod test {
    use crate::{error::IndexError, lens::value::ValueLens};

    use super::*;

    #[test]
    fn test_new_ok() {
        assert!(OverlayLens::new(ValueLens::new(1, Size::new(1, 1).unwrap()), ValueLens::new(1, Size::new(1, 1).unwrap()), Point::new(0, 0).unwrap()).is_ok());
    }

    #[test]
    fn test_new_err() {
        assert!(OverlayLens::new(ValueLens::new(1, Size::new(1, 1).unwrap()), ValueLens::new(1, Size::new(1, 1).unwrap()), Point::new(1, 0).unwrap()).is_err_and(|e| e == OverlayLensCreationError::OverlayStartOutOfBounds));
        assert!(OverlayLens::new(ValueLens::new(1, Size::new(1, 1).unwrap()), ValueLens::new(1, Size::new(1, 1).unwrap()), Point::new(0, 1).unwrap()).is_err_and(|e| e == OverlayLensCreationError::OverlayStartOutOfBounds));
        assert!(OverlayLens::new(ValueLens::new(1, Size::new(1, 1).unwrap()), ValueLens::new(1, Size::new(2, 1).unwrap()), Point::new(0, 0).unwrap()).is_err_and(|e| e == OverlayLensCreationError::OverlayTooBig));
        assert!(OverlayLens::new(ValueLens::new(1, Size::new(1, 1).unwrap()), ValueLens::new(1, Size::new(1, 2).unwrap()), Point::new(0, 0).unwrap()).is_err_and(|e| e == OverlayLensCreationError::OverlayTooBig));
    }

    #[test]
    fn test_look() {
        let overlay_top_left = OverlayLens::new(ValueLens::new(0, Size::new(2, 2).unwrap()), ValueLens::new(1, Size::new(1, 1).unwrap()), Point::new(0, 0).unwrap()).unwrap();
        assert_eq!(overlay_top_left.look(Point::new(0, 0).unwrap()).unwrap(), 1);
        assert_eq!(overlay_top_left.look(Point::new(1, 0).unwrap()).unwrap(), 0);
        assert_eq!(overlay_top_left.look(Point::new(0, 1).unwrap()).unwrap(), 0);
        assert_eq!(overlay_top_left.look(Point::new(1, 1).unwrap()).unwrap(), 0);
        assert_eq!(overlay_top_left.look(Point::new(2, 0).unwrap()).unwrap_err(), IndexError::OutOfBounds);
        assert_eq!(overlay_top_left.look(Point::new(0, 2).unwrap()).unwrap_err(), IndexError::OutOfBounds);

        let overlay_bottom_right = OverlayLens::new(ValueLens::new(0, Size::new(2, 2).unwrap()), ValueLens::new(1, Size::new(1, 1).unwrap()), Point::new(1, 1).unwrap()).unwrap();
        assert_eq!(overlay_bottom_right.look(Point::new(0, 0).unwrap()).unwrap(), 0);
        assert_eq!(overlay_bottom_right.look(Point::new(1, 0).unwrap()).unwrap(), 0);
        assert_eq!(overlay_bottom_right.look(Point::new(0, 1).unwrap()).unwrap(), 0);
        assert_eq!(overlay_bottom_right.look(Point::new(1, 1).unwrap()).unwrap(), 1);
        assert_eq!(overlay_bottom_right.look(Point::new(2, 0).unwrap()).unwrap_err(), IndexError::OutOfBounds);
        assert_eq!(overlay_bottom_right.look(Point::new(0, 2).unwrap()).unwrap_err(), IndexError::OutOfBounds);
    }
}
