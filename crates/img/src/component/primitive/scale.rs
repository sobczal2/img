use std::cmp::Ordering;

use thiserror::Error;

use crate::{
    component::primitive::{
        SizeCreationError,
        SizeCreationResult,
        point::PointCreationResult,
    },
    image::DIMENSION_MAX,
};

use super::{
    Point,
    Size,
};

#[derive(Error, Debug, PartialEq)]
pub enum ScaleCreationError {
    #[error("Scale x value is outside valid range [{min_scale}, {max_scale}]", min_scale = Scale::FACTOR_MIN, max_scale = Scale::FACTOR_MAX)]
    ScaleXInvalid,
    #[error("Scale y value is outside valid range [{min_scale}, {max_scale}]", min_scale = Scale::FACTOR_MIN, max_scale = Scale::FACTOR_MAX)]
    ScaleYInvalid,
}

pub type ScaleCreationResult<T> = Result<T, ScaleCreationError>;

/// Represents a 2D scale with separate x and y scaling factors.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scale(f32, f32);

impl Scale {
    /// Maximum valid scaling factor.
    pub const FACTOR_MAX: f32 = 1e4;

    /// Minimum valid scaling factor.
    pub const FACTOR_MIN: f32 = 1f32 / Self::FACTOR_MAX;

    /// Create a new [`Scale`] with the specified x and y scaling factors.
    ///
    /// Both x and y must be within range <[`Scale::FACTOR_MIN`], [`Scale::FACTOR_MAX`]> inclusive.
    ///
    /// Returns [`Scale`] if both parameters are valid, [`ScaleCreationError`] otherwise.
    ///
    /// # Errors
    ///
    /// * `ScaleCreationError::ScaleXInvalid` - if `x` is not within <[`Scale::MIN`],
    ///   [`Scale::FACTOR_MAX`]>, is [`NAN`], [`INFINITY`] or is [`NEG_INFINITY`]
    /// * `ScaleCreationError::ScaleYInvalid` - if `y` is not within <[`Scale::MIN`],
    ///   [`Scale::FACTOR_MAX`]>, is [`NAN`],[`INFINITY`] or is [`NEG_INFINITY`] 
    ///
    /// [`NAN`]: f32::NAN
    /// [`INFINITY`]: f32::INFINITY
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let normal = Scale::new(1.0, 1.0)?; // Identity scale
    /// let enlarge = Scale::new(2.0, 1.5)?; // Enlarge width 2x, height 1.5x
    /// let shrink = Scale::new(0.5, 0.25)?; // Shrink to half width, quarter height
    ///
    /// // Invalid scales
    /// assert!(Scale::new(0.00009, 1.0).is_err()); // x too small
    /// assert!(Scale::new(1.0, 10000.1).is_err()); // y too large
    /// assert!(Scale::new(f32::NAN, 1.0).is_err()); // NaN not allowed
    /// assert!(Scale::new(f32::INFINITY, 1.0).is_err()); // Infinity not allowed
    /// assert!(Scale::new(f32::NEG_INFINITY, 1.0).is_err()); // Negative Infinity not allowed
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(x: f32, y: f32) -> ScaleCreationResult<Self> {
        let valid_range = Self::FACTOR_MIN..=Self::FACTOR_MAX;
        if !valid_range.contains(&x) {
            return Err(ScaleCreationError::ScaleXInvalid);
        }

        if !valid_range.contains(&y) {
            return Err(ScaleCreationError::ScaleYInvalid);
        }

        Ok(Self(x, y))
    }

    /// Returns the x (horizontal) scaling factor.
    pub fn x(&self) -> f32 {
        self.0
    }

    /// Returns the y (vertical) scaling factor.
    pub fn y(&self) -> f32 {
        self.1
    }

    /// Returns the inverse scale (1/x, 1/y).
    ///
    /// # Examples
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let identity = Scale::new(1.0, 1.0)?;
    /// let half = Scale::new(0.5, 0.5)?;
    /// let uneven = Scale::new(2.0, 1.5)?;
    ///
    /// assert_eq!(identity.inverse(), Scale::new(1.0, 1.0)?);
    /// assert_eq!(half.inverse(), Scale::new(2.0, 2.0)?);
    /// assert_eq!(uneven.inverse(), Scale::new(0.5, 2.0 / 3.0)?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn inverse(&self) -> Scale {
        // SAFETY: safe because inverse is always within range `[Scale::FACTOR_MIN, Scale::FACTOR_MAX]`
        // inclusive.
        Scale(1.0 / self.0, 1.0 / self.1)
    }

    /// Applies the scale transformation to a [`Size`], returning a new scaled [`Size`].
    /// Rounds results to value closer to zero.
    ///
    /// Returns scaled [`Size`] or [`SizeCreationError`] if resulting Size would not
    /// be valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let identity = Scale::new(1.0, 1.0)?;
    /// let half = Scale::new(0.5, 0.5)?;
    /// let uneven = Scale::new(2.0, 1.5)?;
    ///
    /// let size = Size::new(100, 100)?;
    ///
    /// assert_eq!(identity.apply(size)?, Size::new(100, 100)?);
    /// assert_eq!(half.apply(size)?, Size::new(50, 50)?);
    /// assert_eq!(uneven.apply(size)?, Size::new(200, 150)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn apply(&self, size: Size) -> SizeCreationResult<Size> {
        if size.width() as f32 > DIMENSION_MAX as f32 / self.0 {
            return Err(SizeCreationError::WidthTooBig);
        }

        if size.height() as f32 > DIMENSION_MAX as f32 / self.1 {
            return Err(SizeCreationError::HeightTooBig);
        }

        let new_width = size.width() as f64 * self.0 as f64;
        let new_height = size.height() as f64 * self.1 as f64;

        Size::new(new_width.floor() as usize, new_height.floor() as usize)
    }

    /// Transform the point to scaled coordinate space.
    /// Rounds results to value closer to zero.
    ///
    /// Returns scaled [`Point`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let identity = Scale::new(1.0, 1.0)?;
    /// let half = Scale::new(0.5, 0.5)?;
    /// let uneven = Scale::new(2.0, 1.5)?;
    ///
    /// let point = Point::new(100, 100)?;
    ///
    /// assert_eq!(identity.translate(point)?, Point::new(100, 100)?);
    /// assert_eq!(half.translate(point)?, Point::new(50, 50)?);
    /// assert_eq!(uneven.translate(point)?, Point::new(200, 150)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate(&self, point: Point) -> PointCreationResult<Point> {
        let new_x = point.x() as f64 * self.0 as f64;
        let new_y = point.y() as f64 * self.1 as f64;

        Point::new(new_x.floor() as usize, new_y.floor() as usize)
    }
}

/// [`Eq`] can be safely implemented since we guarantee that [`Scale`] has floats within range
/// <[`Scale::FACTOR_MIN`], [`Scale::FACTOR_MAX`]>.
impl Eq for Scale {}

impl PartialOrd for Scale {
    /// Returns [`Ordering`] of scales or [`None`] if it is not possible to compare them.
    ///
    /// A scale `a` is less than or equal to `b` if both `x` and `y` components
    /// are less than or equal. If one component is greater and other is smaller
    /// then it returns [`None`].
    ///
    /// # Examples
    /// ```
    /// use img::prelude::*;
    /// use std::cmp::Ordering;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(
    ///     Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(10.0, 10.0)?),
    ///     Some(Ordering::Equal)
    /// );
    /// assert_eq!(Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(20.0, 20.0)?), Some(Ordering::Less));
    /// assert_eq!(Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(10.0, 20.0)?), Some(Ordering::Less));
    /// assert_eq!(Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(20.0, 10.0)?), Some(Ordering::Less));
    /// assert_eq!(
    ///     Scale::new(20.0, 20.0)?.partial_cmp(&Scale::new(10.0, 10.0)?),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Scale::new(20.0, 10.0)?.partial_cmp(&Scale::new(10.0, 10.0)?),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(
    ///     Scale::new(10.0, 20.0)?.partial_cmp(&Scale::new(10.0, 10.0)?),
    ///     Some(Ordering::Greater)
    /// );
    /// assert_eq!(Scale::new(20.0, 10.0)?.partial_cmp(&Scale::new(10.0, 20.0)?), None);
    /// assert_eq!(Scale::new(10.0, 20.0)?.partial_cmp(&Scale::new(20.0, 10.0)?), None);
    /// # Ok(())
    /// # }
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        if self.0 <= other.0 && self.1 <= other.1 {
            return Some(Ordering::Less);
        }

        if self.0 >= other.0 && self.1 >= other.1 {
            return Some(Ordering::Greater);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use core::f32;
    use crate::component::primitive::PointCreationError;

    use super::*;

    #[test]
    fn test_new_ok() {
        assert!(Scale::new(1f32, 1f32).is_ok());
        assert!(Scale::new(Scale::FACTOR_MIN, 1f32).is_ok());
        assert!(Scale::new(Scale::FACTOR_MAX, 1f32).is_ok());
        assert!(Scale::new(1f32, Scale::FACTOR_MIN).is_ok());
        assert!(Scale::new(1f32, Scale::FACTOR_MAX).is_ok());
    }

    #[test]
    fn test_new_err() {
        assert_eq!(Scale::new(Scale::FACTOR_MIN - 1f32, 1f32).unwrap_err(), ScaleCreationError::ScaleXInvalid);
        assert_eq!(Scale::new(1f32, Scale::FACTOR_MIN - 1f32).unwrap_err(), ScaleCreationError::ScaleYInvalid);
        assert_eq!(Scale::new(Scale::FACTOR_MIN - 1f32, Scale::FACTOR_MIN - 1f32).unwrap_err(), ScaleCreationError::ScaleXInvalid);

        assert_eq!(Scale::new(Scale::FACTOR_MAX + 1f32, 1f32).unwrap_err(), ScaleCreationError::ScaleXInvalid);
        assert_eq!(Scale::new(1f32, Scale::FACTOR_MAX + 1f32).unwrap_err(), ScaleCreationError::ScaleYInvalid);
        assert_eq!(Scale::new(Scale::FACTOR_MAX + 1f32, Scale::FACTOR_MAX + 1f32).unwrap_err(), ScaleCreationError::ScaleXInvalid);

        assert_eq!(Scale::new(f32::INFINITY, 1f32).unwrap_err(), ScaleCreationError::ScaleXInvalid);
        assert_eq!(Scale::new(1f32, f32::INFINITY).unwrap_err(), ScaleCreationError::ScaleYInvalid);
        assert_eq!(Scale::new(f32::NEG_INFINITY, 1f32).unwrap_err(), ScaleCreationError::ScaleXInvalid);
        assert_eq!(Scale::new(1f32, f32::NEG_INFINITY).unwrap_err(), ScaleCreationError::ScaleYInvalid);
        assert_eq!(Scale::new(f32::NAN, 1f32).unwrap_err(), ScaleCreationError::ScaleXInvalid);
        assert_eq!(Scale::new(1f32, f32::NAN).unwrap_err(), ScaleCreationError::ScaleYInvalid);
    }

    #[test]
    fn test_inverse() {
        assert_eq!(Scale::new(Scale::FACTOR_MAX, 1f32).unwrap().inverse(), Scale::new(Scale::FACTOR_MIN, 1f32).unwrap());
        assert_eq!(Scale::new(Scale::FACTOR_MIN, 1f32).unwrap().inverse(), Scale::new(Scale::FACTOR_MAX, 1f32).unwrap());
        assert_eq!(Scale::new(1f32, Scale::FACTOR_MAX).unwrap().inverse(), Scale::new(1f32, Scale::FACTOR_MIN).unwrap());
        assert_eq!(Scale::new(1f32, Scale::FACTOR_MIN).unwrap().inverse(), Scale::new(1f32, Scale::FACTOR_MAX).unwrap());
    }

    #[test]
    fn test_apply_ok() {
        assert_eq!(Scale::new(1f32, 1f32).unwrap().apply(Size::new(1, 1).unwrap()).unwrap(), Size::new(1, 1).unwrap());
        assert_eq!(Scale::new(2f32, 1f32).unwrap().apply(Size::new(1, 1).unwrap()).unwrap(), Size::new(2, 1).unwrap());
        assert_eq!(Scale::new(1f32, 2f32).unwrap().apply(Size::new(1, 1).unwrap()).unwrap(), Size::new(1, 2).unwrap());
        assert_eq!(Scale::new(0.5f32, 0.5f32).unwrap().apply(Size::new(2, 2).unwrap()).unwrap(), Size::new(1, 1).unwrap());
        assert_eq!(Scale::new(1f32, 1f32).unwrap().apply(Size::new(DIMENSION_MAX, 1).unwrap()).unwrap(), Size::new(DIMENSION_MAX, 1).unwrap());
        assert_eq!(Scale::new(1f32, 1f32).unwrap().apply(Size::new(1, DIMENSION_MAX).unwrap()).unwrap(), Size::new(1, DIMENSION_MAX).unwrap());
        assert_eq!(Scale::new(1f32, 1f32).unwrap().apply(Size::new(DIMENSION_MAX, DIMENSION_MAX).unwrap()).unwrap(), Size::new(DIMENSION_MAX, DIMENSION_MAX).unwrap());
    }

    #[test]
    fn test_apply_err() {
        assert_eq!(Scale::new(2f32, 1f32).unwrap().apply(Size::new(DIMENSION_MAX / 2 + 1, 1).unwrap()).unwrap_err(), SizeCreationError::WidthTooBig);
        assert_eq!(Scale::new(1f32, 2f32).unwrap().apply(Size::new(1, DIMENSION_MAX / 2 + 1).unwrap()).unwrap_err(), SizeCreationError::HeightTooBig);
        assert_eq!(Scale::new(0.5f32, 1f32).unwrap().apply(Size::new(1, 1).unwrap()).unwrap_err(), SizeCreationError::WidthZero);
        assert_eq!(Scale::new(1f32, 0.5f32).unwrap().apply(Size::new(1, 1).unwrap()).unwrap_err(), SizeCreationError::HeightZero);
    }

    #[test]
    fn test_translate_ok() {
        assert_eq!(Scale::new(1f32, 1f32).unwrap().translate(Point::new(1, 1).unwrap()).unwrap(), Point::new(1, 1).unwrap());
        assert_eq!(Scale::new(0.5f32, 1f32).unwrap().translate(Point::new(1, 1).unwrap()).unwrap(), Point::new(0, 1).unwrap());
        assert_eq!(Scale::new(1f32, 0.5f32).unwrap().translate(Point::new(1, 1).unwrap()).unwrap(), Point::new(1, 0).unwrap());
        assert_eq!(Scale::new(2f32, 1f32).unwrap().translate(Point::new(1, 1).unwrap()).unwrap(), Point::new(2, 1).unwrap());
        assert_eq!(Scale::new(1f32, 2f32).unwrap().translate(Point::new(1, 1).unwrap()).unwrap(), Point::new(1, 2).unwrap());
        assert_eq!(Scale::new(1.5f32, 1f32).unwrap().translate(Point::new(1, 1).unwrap()).unwrap(), Point::new(1, 1).unwrap());
        assert_eq!(Scale::new(1f32, 1.5f32).unwrap().translate(Point::new(1, 1).unwrap()).unwrap(), Point::new(1, 1).unwrap());
        assert_eq!(Scale::new(1f32, 1f32).unwrap().translate(Point::new(DIMENSION_MAX - 1, 1).unwrap()).unwrap(), Point::new(DIMENSION_MAX - 1, 1).unwrap());
        assert_eq!(Scale::new(1f32, 1f32).unwrap().translate(Point::new(1, DIMENSION_MAX - 1).unwrap()).unwrap(), Point::new(1, DIMENSION_MAX - 1).unwrap());
        // TODO: checks for large numbers
    }

    #[test]
    fn test_translate_err() {
        assert_eq!(Scale::new(1.5f32, 1f32).unwrap().translate(Point::new(DIMENSION_MAX - 1, 1).unwrap()).unwrap_err(), PointCreationError::XTooBig);
        assert_eq!(Scale::new(1f32, 1.5f32).unwrap().translate(Point::new(1, DIMENSION_MAX - 1).unwrap()).unwrap_err(), PointCreationError::YTooBig);
    }

    #[test]
    fn test_partial_cmp() {
        assert_eq!(
            Scale::new(1f32, 1f32).unwrap().partial_cmp(&Scale::new(1f32, 1f32).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MAX, 1f32)
                .unwrap()
                .partial_cmp(&Scale::new(Scale::FACTOR_MAX, 1f32).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MIN, 1f32)
                .unwrap()
                .partial_cmp(&Scale::new(Scale::FACTOR_MIN, 1f32).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Scale::new(1f32, Scale::FACTOR_MAX)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, Scale::FACTOR_MAX).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Scale::new(1f32, Scale::FACTOR_MIN)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, Scale::FACTOR_MIN).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MAX, Scale::FACTOR_MAX)
                .unwrap()
                .partial_cmp(&Scale::new(Scale::FACTOR_MAX, Scale::FACTOR_MAX).unwrap()),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MIN, Scale::FACTOR_MIN)
                .unwrap()
                .partial_cmp(&Scale::new(Scale::FACTOR_MIN, Scale::FACTOR_MIN).unwrap()),
            Some(Ordering::Equal)
        );

        assert_eq!(
            Scale::new(Scale::FACTOR_MIN, 1f32)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, 1f32).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Scale::new(1f32, Scale::FACTOR_MIN)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, 1f32).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MIN, Scale::FACTOR_MIN)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, 1f32).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MIN, Scale::FACTOR_MIN)
                .unwrap()
                .partial_cmp(&Scale::new(Scale::FACTOR_MIN, 1f32).unwrap()),
            Some(Ordering::Less)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MIN, Scale::FACTOR_MIN)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, Scale::FACTOR_MIN).unwrap()),
            Some(Ordering::Less)
        );

        assert_eq!(
            Scale::new(Scale::FACTOR_MAX, 1f32)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, 1f32).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Scale::new(1f32, Scale::FACTOR_MAX)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, 1f32).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MAX, Scale::FACTOR_MAX)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, 1f32).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MAX, Scale::FACTOR_MAX)
                .unwrap()
                .partial_cmp(&Scale::new(Scale::FACTOR_MAX, 1f32).unwrap()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Scale::new(Scale::FACTOR_MAX, Scale::FACTOR_MAX)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, Scale::FACTOR_MAX).unwrap()),
            Some(Ordering::Greater)
        );

        assert_eq!(
            Scale::new(Scale::FACTOR_MAX, 1f32)
                .unwrap()
                .partial_cmp(&Scale::new(1f32, Scale::FACTOR_MAX).unwrap()),
            None
        );
        assert_eq!(
            Scale::new(1f32, Scale::FACTOR_MAX)
                .unwrap()
                .partial_cmp(&Scale::new(Scale::FACTOR_MAX, 1f32).unwrap()),
            None
        );
    }
}
