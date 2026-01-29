use thiserror::Error;

use crate::pixel::{
    Pixel,
    PixelRgbaf32,
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("hue value is invalid")]
    HueInvalid,
    #[error("saturation value is invalid")]
    SaturationInvalid,
    #[error("value value is invalid")]
    ValueInvalid,
}

pub type CreationResult = std::result::Result<HsvPixel, CreationError>;

/// A `struct` representing a pixel in HSV (Hue, Saturation, Value) color space.
///
/// - `hue`: 0-360 degrees
/// - `saturation`: 0-1 range
/// - `value`: 0-1 range (brightness)
/// - `alpha`: 0-255 (opacity)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsvPixel {
    hue: f32,
    saturation: f32,
    value: f32,
    alpha: u8,
}
impl Eq for HsvPixel {}

impl HsvPixel {
    /// Create a [`HsvPixel`] from given hue, saturation, value and alpha.
    ///
    /// Returns [`HsvPixel`] if hue, saturation and value are valid, [`CreationError`]
    /// otherwise.
    ///
    /// # Arguments
    ///
    /// * `hue` - Hue in degrees (0-360)
    /// * `saturation` - Saturation (0-1)
    /// * `value` - Value/brightness (0-1)
    /// * `alpha` - Alpha/opacity (0-255)
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// // Valid pixel (red at full saturation and brightness)
    /// let red = HsvPixel::new(0.0, 1.0, 1.0, 255);
    /// assert!(red.is_ok());
    ///
    /// // Invalid hue (must be 0-360)
    /// let invalid_hue = HsvPixel::new(400.0, 1.0, 1.0, 255);
    /// assert!(invalid_hue.is_err());
    ///
    /// // Invalid saturation (must be 0-1)
    /// let invalid_sat = HsvPixel::new(0.0, 2.0, 1.0, 255);
    /// assert!(invalid_sat.is_err());
    ///
    /// // Invalid value (must be 0-1)
    /// let invalid_val = HsvPixel::new(0.0, 1.0, -0.5, 255);
    /// assert!(invalid_val.is_err());
    /// ```
    pub fn new(hue: f32, saturation: f32, value: f32, alpha: u8) -> CreationResult {
        if !(0f32..=360f32).contains(&hue) {
            return Err(CreationError::HueInvalid);
        }

        if !(0f32..=1f32).contains(&saturation) {
            return Err(CreationError::SaturationInvalid);
        }

        if !(0f32..=1f32).contains(&value) {
            return Err(CreationError::ValueInvalid);
        }

        Ok(HsvPixel { hue, saturation, value, alpha })
    }

    /// Get hue component (0-360 degrees).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let pixel = HsvPixel::new(180.0, 0.5, 0.8, 255).unwrap();
    /// assert!((pixel.hue() - 180.0).abs() < 1e-6);
    /// ```
    pub fn hue(&self) -> f32 {
        self.hue
    }

    /// Get saturation component (0-1).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let pixel = HsvPixel::new(180.0, 0.5, 0.8, 255).unwrap();
    /// assert!((pixel.saturation() - 0.5).abs() < 1e-6);
    /// ```
    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    /// Get value component (0-1).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let pixel = HsvPixel::new(180.0, 0.5, 0.8, 255).unwrap();
    /// assert!((pixel.value() - 0.8).abs() < 1e-6);
    /// ```
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get alpha component (0-255).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let pixel = HsvPixel::new(180.0, 0.5, 0.8, 200).unwrap();
    /// assert_eq!(pixel.alpha(), 200);
    /// ```
    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    /// Get 0-1 normalized alpha component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let pixel = HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap();
    /// assert!((pixel.alpha_f32() - 1.0).abs() < 1e-6);
    ///
    /// let pixel = HsvPixel::new(0.0, 1.0, 1.0, 128).unwrap();
    /// assert!((pixel.alpha_f32() - 0.502).abs() < 1e-2);
    /// ```
    pub fn alpha_f32(&self) -> f32 {
        self.alpha as f32 / 255f32
    }

    /// Set hue component (0-360 degrees).
    ///
    /// Note: This does not validate the input. For validated creation, use [`HsvPixel::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let mut pixel = HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap();
    /// pixel.set_hue(120.0);
    /// assert!((pixel.hue() - 120.0).abs() < 1e-6);
    /// ```
    pub fn set_hue(&mut self, value: f32) {
        self.hue = value;
    }

    /// Set saturation component (0-1).
    ///
    /// Note: This does not validate the input. For validated creation, use [`HsvPixel::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let mut pixel = HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap();
    /// pixel.set_saturation(0.5);
    /// assert!((pixel.saturation() - 0.5).abs() < 1e-6);
    /// ```
    pub fn set_saturation(&mut self, value: f32) {
        self.saturation = value;
    }

    /// Set value component (0-1).
    ///
    /// Note: This does not validate the input. For validated creation, use [`HsvPixel::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let mut pixel = HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap();
    /// pixel.set_value(0.5);
    /// assert!((pixel.value() - 0.5).abs() < 1e-6);
    /// ```
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    /// Set alpha component (0-255).
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let mut pixel = HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap();
    /// pixel.set_alpha(128);
    /// assert_eq!(pixel.alpha(), 128);
    /// ```
    pub fn set_alpha(&mut self, value: u8) {
        self.alpha = value;
    }

    /// Set 0-1 normalized alpha component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::hsv::HsvPixel;
    ///
    /// let mut pixel = HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap();
    /// pixel.set_alpha_f32(0.5);
    /// assert_eq!(pixel.alpha(), 128);
    ///
    /// // Values are clamped
    /// pixel.set_alpha_f32(2.0);
    /// assert_eq!(pixel.alpha(), 255);
    /// ```
    pub fn set_alpha_f32(&mut self, value: f32) {
        self.alpha = (value * 255f32).round().clamp(0f32, 255f32) as u8;
    }
}

impl From<Pixel> for HsvPixel {
    /// Convert `Pixel` to `HsvPixel`. This effectively converts RGB color space to HSV.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::pixel::{
    ///     Pixel,
    ///     hsv::HsvPixel,
    /// };
    /// macro_rules! assert_hsv_pixel_eq {
    ///     ($left:expr, $right:expr) => {
    ///         assert!(
    ///             ($left.hue() - $right.hue()).abs() < 1e-2,
    ///             "hue: expected {}, got {}",
    ///             $right.hue(),
    ///             $left.hue()
    ///         );
    ///         assert!(
    ///             ($left.saturation() - $right.saturation()).abs() < 1e-2,
    ///             "saturation: expected {}, got {}",
    ///             $right.saturation(),
    ///             $left.saturation()
    ///         );
    ///         assert!(
    ///             ($left.value() - $right.value()).abs() < 1e-2,
    ///             "value: expected {}, got {}",
    ///             $right.value(),
    ///             $left.value()
    ///         );
    ///         assert_eq!(
    ///             $left.alpha(),
    ///             $right.alpha(),
    ///             "alpha: expected {}, got {}",
    ///             $right.alpha(),
    ///             $left.alpha()
    ///         );
    ///     };
    /// }
    ///
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::from(Pixel::new([255, 0, 0, 255])),
    ///     HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap()
    /// );
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::from(Pixel::new([0, 255, 0, 255])),
    ///     HsvPixel::new(120.0, 1.0, 1.0, 255).unwrap()
    /// );
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::from(Pixel::new([0, 0, 255, 255])),
    ///     HsvPixel::new(240.0, 1.0, 1.0, 255).unwrap()
    /// );
    /// ```
    fn from(value: Pixel) -> Self {
        let r = value.r_f32();
        let g = value.g_f32();
        let b = value.b_f32();
        let a = value.a();

        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);

        let delta = cmax - cmin;

        let mut hue = if delta == 0f32 {
            0f32
        } else if r == cmax {
            60f32 * (((g - b) / delta) % 6f32)
        } else if g == cmax {
            60f32 * (((b - r) / delta) + 2f32)
        } else {
            60f32 * (((r - g) / delta) + 4f32)
        };
        if hue < 0f32 {
            hue += 360f32
        };

        let saturation = if cmax != 0f32 { delta / cmax } else { 0f32 };

        let value = cmax;

        Self { hue, saturation, value, alpha: a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixel::Pixel;

    macro_rules! assert_hsv_pixel_eq {
        ($left:expr, $right:expr) => {
            assert!(
                ($left.hue() - $right.hue()).abs() < 1e-2,
                "hue: expected {}, got {}",
                $right.hue(),
                $left.hue()
            );
            assert!(
                ($left.saturation() - $right.saturation()).abs() < 1e-2,
                "saturation: expected {}, got {}",
                $right.saturation(),
                $left.saturation()
            );
            assert!(
                ($left.value() - $right.value()).abs() < 1e-2,
                "value: expected {}, got {}",
                $right.value(),
                $left.value()
            );
            assert_eq!(
                $left.alpha(),
                $right.alpha(),
                "alpha: expected {}, got {}",
                $right.alpha(),
                $left.alpha()
            );
        };
    }

    #[test]
    fn test_new_ok() {
        // Valid cases at boundaries
        assert!(HsvPixel::new(0.0, 0.0, 0.0, 0).is_ok());
        assert!(HsvPixel::new(360.0, 1.0, 1.0, 255).is_ok());
        assert!(HsvPixel::new(180.0, 0.5, 0.5, 128).is_ok());
    }

    #[test]
    fn test_new_err_hue() {
        // Hue must be 0-360
        assert!(matches!(
            HsvPixel::new(-0.1, 0.5, 0.5, 255).unwrap_err(),
            CreationError::HueInvalid
        ));
        assert!(matches!(
            HsvPixel::new(360.1, 0.5, 0.5, 255).unwrap_err(),
            CreationError::HueInvalid
        ));
    }

    #[test]
    fn test_new_err_saturation() {
        // Saturation must be 0-1
        assert!(matches!(
            HsvPixel::new(180.0, -0.1, 0.5, 255).unwrap_err(),
            CreationError::SaturationInvalid
        ));
        assert!(matches!(
            HsvPixel::new(180.0, 1.1, 0.5, 255).unwrap_err(),
            CreationError::SaturationInvalid
        ));
    }

    #[test]
    fn test_new_err_value() {
        // Value must be 0-1
        assert!(matches!(
            HsvPixel::new(180.0, 0.5, -0.1, 255).unwrap_err(),
            CreationError::ValueInvalid
        ));
        assert!(matches!(
            HsvPixel::new(180.0, 0.5, 1.1, 255).unwrap_err(),
            CreationError::ValueInvalid
        ));
    }

    #[test]
    fn test_getters() {
        let pixel = HsvPixel::new(120.0, 0.8, 0.6, 200).unwrap();
        assert!((pixel.hue() - 120.0).abs() < 1e-6);
        assert!((pixel.saturation() - 0.8).abs() < 1e-6);
        assert!((pixel.value() - 0.6).abs() < 1e-6);
        assert_eq!(pixel.alpha(), 200);
    }

    #[test]
    fn test_setters() {
        let mut pixel = HsvPixel::new(0.0, 0.0, 0.0, 0).unwrap();

        pixel.set_hue(180.0);
        assert!((pixel.hue() - 180.0).abs() < 1e-6);

        pixel.set_saturation(0.5);
        assert!((pixel.saturation() - 0.5).abs() < 1e-6);

        pixel.set_value(0.7);
        assert!((pixel.value() - 0.7).abs() < 1e-6);

        pixel.set_alpha(128);
        assert_eq!(pixel.alpha(), 128);
    }

    #[test]
    fn test_alpha_f32() {
        let pixel_full = HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap();
        assert!((pixel_full.alpha_f32() - 1.0).abs() < 1e-6);

        let pixel_zero = HsvPixel::new(0.0, 1.0, 1.0, 0).unwrap();
        assert!((pixel_zero.alpha_f32() - 0.0).abs() < 1e-6);

        let pixel_half = HsvPixel::new(0.0, 1.0, 1.0, 128).unwrap();
        assert!((pixel_half.alpha_f32() - 0.502).abs() < 1e-2);
    }

    #[test]
    fn test_set_alpha_f32() {
        let mut pixel = HsvPixel::new(0.0, 1.0, 1.0, 0).unwrap();

        pixel.set_alpha_f32(0.5);
        assert_eq!(pixel.alpha(), 128);

        pixel.set_alpha_f32(1.0);
        assert_eq!(pixel.alpha(), 255);

        pixel.set_alpha_f32(0.0);
        assert_eq!(pixel.alpha(), 0);
    }

    #[test]
    fn test_set_alpha_f32_clamping() {
        let mut pixel = HsvPixel::new(0.0, 1.0, 1.0, 128).unwrap();

        // Upper clamping
        pixel.set_alpha_f32(2.0);
        assert_eq!(pixel.alpha(), 255);

        // Lower clamping
        pixel.set_alpha_f32(-1.0);
        assert_eq!(pixel.alpha(), 0);
    }

    #[test]
    fn test_equality() {
        let p1 = HsvPixel::new(120.0, 0.5, 0.8, 200).unwrap();
        let p2 = HsvPixel::new(120.0, 0.5, 0.8, 200).unwrap();
        let p3 = HsvPixel::new(121.0, 0.5, 0.8, 200).unwrap();

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_clone() {
        let p1 = HsvPixel::new(120.0, 0.5, 0.8, 200).unwrap();
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_hsv_pixel_from_pixel() {
        let cases = vec![
            // r, g, b, a, expected_h, expected_s, expected_v, expected_a
            (0, 0, 0, 255, 0.0f32, 0.0f32, 0.0f32, 255),
            (0, 0, 0, 128, 0.0f32, 0.0f32, 0.0f32, 128),
            (0, 0, 0, 0, 0.0f32, 0.0f32, 0.0f32, 0),
            (255, 255, 255, 255, 0.0f32, 0.0f32, 1.0f32, 255),
            (255, 0, 0, 255, 0.0f32, 1.0f32, 1.0f32, 255),
            (0, 255, 0, 255, 120.0f32, 1.0f32, 1.0f32, 255),
            (0, 0, 255, 255, 240.0f32, 1.0f32, 1.0f32, 255),
            (255, 255, 0, 255, 60.0f32, 1.0f32, 1.0f32, 255),
            (0, 255, 255, 255, 180.0f32, 1.0f32, 1.0f32, 255),
            (255, 0, 255, 255, 300.0f32, 1.0f32, 1.0f32, 255),
            (128, 128, 128, 255, 0.0f32, 0.0f32, 0.5f32, 255),
            (128, 0, 0, 255, 0.0f32, 1.0f32, 0.5f32, 255),
            (255, 191, 191, 255, 0.0f32, 0.25f32, 1.0f32, 255),
            (128, 128, 0, 255, 60.0f32, 1.0f32, 0.5f32, 255),
            (0, 128, 128, 255, 180.0f32, 1.0f32, 0.5f32, 255),
            (128, 0, 128, 255, 300.0f32, 1.0f32, 0.5f32, 255),
        ];

        for (r, g, b, a, exp_h, exp_s, exp_v, exp_a) in cases {
            let pixel = Pixel::new([r, g, b, a]);
            let hsv = HsvPixel::from(pixel);
            let expected_hsv = HsvPixel::new(exp_h, exp_s, exp_v, exp_a).unwrap();
            assert_hsv_pixel_eq!(hsv, expected_hsv);
        }
    }
}
