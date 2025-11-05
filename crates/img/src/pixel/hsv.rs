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

    /// Get hue component.
    pub fn hue(&self) -> f32 {
        self.hue
    }

    /// Get saturation component.
    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    /// Get value component.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get alpha component.
    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    /// Get 0-1 normalized alpha component.
    pub fn alpha_f32(&self) -> f32 {
        self.alpha as f32 / 255f32
    }

    /// Set hue component.
    pub fn set_hue(&mut self, value: f32) {
        self.hue = value;
    }

    /// Set saturation component.
    pub fn set_saturation(&mut self, value: f32) {
        self.saturation = value;
    }

    /// Set value component.
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    /// Set alpha component.
    pub fn set_alpha(&mut self, value: u8) {
        self.alpha = value;
    }

    /// Set 0-1 normalized alpha component.
    ///
    /// This clamps the result if it is not in 0-1 range.
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
    fn hsv_pixel_from_pixel() {
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
