use std::cmp::Ordering;

use thiserror::Error;

use crate::primitive::{
    point::Point,
    size::{Size, SizeCreationError},
};

#[derive(Error, Debug)]
pub enum ScaleCreationError {
    #[error("Scale x value {0} is outside valid range [{min_scale}, {max_scale}]", min_scale = Scale::MIN, max_scale = Scale::MAX)]
    ScaleXInvalid(f32),
    #[error("Scale y value {0} is outside valid range [{min_scale}, {max_scale}]", min_scale = Scale::MIN, max_scale = Scale::MAX)]
    ScaleYInvalid(f32),
}

/// Represents a 2D scale with separate x and y scaling factors.
///
/// # Examples
/// ```
/// use img::primitive::scale::Scale;
///
/// // Crate a scale that doubles width and triples height
/// let scale = Scale::new(2.0, 3.0).unwrap();
///
/// // Create a scale that halves width and height
/// let half_scale = Scale::new(0.5, 0.5).unwrap();
///
/// // Tries to create scale, but x value is invalid
/// let invalid_scale = Scale::new(0.00001, 1.0).unwrap_err(); // return ScaleCreationError::ScaleXInvalid
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Scale(f32, f32);

impl Scale {
    /// Minimum valid scaling factor.
    pub const MIN: f32 = 1e-4;

    /// Maximum valid scaling factor.
    pub const MAX: f32 = 1f32 / Self::MIN;

    /// Create a new `Scale` with the specified x and y scaling factors.
    ///
    /// Both x and y must be within range `[Scale::MIN, Scale::MAX]` inclusive.
    ///
    /// Returns `Ok(Scale)` if both parameters are valid, otherwise returns a
    /// `ScaleCreationError`.
    ///
    /// # Errors
    ///
    /// * `ScaleCreationError::ScaleXInvalid` - if `x` is not within `[Scale::MIN, Scale::MAX]`,
    ///   is `NAN`, or is `INFINITE`
    /// * `ScaleCreationError::ScaleYInvalid` - if `y` is not within `[Scale::MIN, Scale::MAX]`,
    ///   is `NAN`, or is `INFINITE`
    ///
    /// # Examples
    ///
    /// ```
    /// use img::primitive::scale::Scale;
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
    /// assert!(Scale::new(f32::INFINITY, 1.0).is_err()); // NaN not allowed
    ///
    /// # Ok(())
    /// }
    /// ```
    pub fn new(x: f32, y: f32) -> Result<Self, ScaleCreationError> {
        let valid_range = Self::MIN..=Self::MAX;
        if !valid_range.contains(&x) {
            return Err(ScaleCreationError::ScaleXInvalid(x));
        }

        if !valid_range.contains(&y) {
            return Err(ScaleCreationError::ScaleYInvalid(y));
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
    /// use img::primitive::scale::Scale;
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
        // SAFETY: safe because inverse is always within range `[Scale::MIN, Scale::MAX]`
        // inclusive.
        Scale(1.0 / self.0, 1.0 / self.1)
    }

    /// Applies the scale transformation to a `Size`, returning a new scaled `Size`.
    /// Rounds results to the nearest integer or further from zero if value is in the
    /// middle.
    ///
    /// Returns scaled `Ok(Size)` or `SizeCreationError` if resulting Size would not
    /// be valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::primitive::{size::Size, scale::Scale};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let identity = Scale::new(1.0, 1.0)?;
    /// let half = Scale::new(0.5, 0.5)?;
    /// let uneven = Scale::new(2.0, 1.5)?;
    ///
    /// let size = Size::from_usize(100, 100)?;
    ///
    /// assert_eq!(identity.apply(size)?, Size::from_usize(100, 100)?);
    /// assert_eq!(half.apply(size)?, Size::from_usize(50, 50)?);
    /// assert_eq!(uneven.apply(size)?, Size::from_usize(200, 150)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn apply(&self, size: Size) -> Result<Size, SizeCreationError> {
        let new_width: f32 = size.width() as f32 * self.0;
        let new_height: f32 = size.height() as f32 * self.1;

        Size::from_usize(new_width.round() as usize, new_height.round() as usize)
    }

    /// Transform the point to scaled coordinate space.
    /// Rounds results to the nearest integer or further from zero if value is in the
    /// middle.
    ///
    /// Returns scaled `Point`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::primitive::{point::Point, scale::Scale};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let identity = Scale::new(1.0, 1.0)?;
    /// let half = Scale::new(0.5, 0.5)?;
    /// let uneven = Scale::new(2.0, 1.5)?;
    ///
    /// let point = Point::new(100, 100);
    ///
    /// assert_eq!(identity.translate(point), Point::new(100, 100));
    /// assert_eq!(half.translate(point), Point::new(50, 50));
    /// assert_eq!(uneven.translate(point), Point::new(200, 150));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn translate(&self, point: Point) -> Point {
        let new_x = point.x() as f32 * self.0;
        let new_y = point.y() as f32 * self.1;

        Point::new(new_x.round() as usize, new_y.round() as usize)
    }
}

impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

/// `Eq` can be safely implemented since we guarantee that `Scale` has floats within range
/// `[Scale::MIN, Scale::MAX]`.
impl Eq for Scale {}

impl PartialOrd for Scale {
    /// Returns ordering of scales or none if it is not possible to compare them.
    ///
    /// A scale `a` is less than or equal to `b` if both `x` and `y` components
    /// are less than or equal. If one component is greater and other is smaller
    /// then it returns `None`.
    ///
    /// # Examples
    /// ```
    /// use img::primitive::scale::Scale;
    /// use std::cmp::Ordering;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(10.0, 10.0)?), Some(Ordering::Equal));
    /// assert_eq!(Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(20.0, 20.0)?), Some(Ordering::Less));
    /// assert_eq!(Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(10.0, 20.0)?), Some(Ordering::Less));
    /// assert_eq!(Scale::new(10.0, 10.0)?.partial_cmp(&Scale::new(20.0, 10.0)?), Some(Ordering::Less));
    /// assert_eq!(Scale::new(20.0, 20.0)?.partial_cmp(&Scale::new(10.0, 10.0)?),
    /// Some(Ordering::Greater));
    /// assert_eq!(Scale::new(20.0, 10.0)?.partial_cmp(&Scale::new(10.0, 10.0)?),
    /// Some(Ordering::Greater));
    /// assert_eq!(Scale::new(10.0, 20.0)?.partial_cmp(&Scale::new(10.0, 10.0)?),
    /// Some(Ordering::Greater));
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
