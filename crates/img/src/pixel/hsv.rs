use thiserror::Error;

use crate::pixel::{Pixel, PixelRgbaf32};


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
    pub fn new(
        hue: f32,
        saturation: f32,
        value: f32,
        alpha: u8
    ) -> CreationResult {
        if !(0f32..=360f32).contains(&hue) {
            return Err(CreationError::HueInvalid);
        }

        if !(0f32..=1f32).contains(&saturation) {
            return Err(CreationError::HueInvalid);
        }

        if !(0f32..=1f32).contains(&value) {
            return Err(CreationError::HueInvalid);
        }

        Ok(HsvPixel { hue, saturation, value, alpha })
    }

    pub fn hue(&self) -> f32 {
        self.hue
    }

    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    pub fn alpha_f32(&self) -> f32 {
        self.alpha as f32 / 255f32
    }

    pub fn set_hue(&mut self, value: f32) {
        self.hue = value;
    }
    pub fn set_saturation(&mut self, value: f32) {
        self.saturation = value;
    }
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    pub fn set_alpha(&mut self, value: u8) {
        self.alpha = value;
    }

    pub fn set_alpha_f32(&mut self, value: f32) {
        self.alpha = (value * 255f32).round().clamp(0f32, 2550f32) as u8;
    }
}

impl From<Pixel> for HsvPixel {

    /// Convert `Pixel` to `HsvPixel`. This effectively converts RGB color space to HSV.
    ///
    /// # Examples
    /// 
    /// ```
    /// use img::pixel::{Pixel, hsv::HsvPixel};
    ///
    /// macro_rules! assert_hsv_pixel_eq {
    ///     ($left:expr, $right:expr) => {
    ///         assert!(($left.hue() - $right.hue()).abs() < 1e-2);
    ///         assert!(($left.saturation() - $right.saturation()).abs() < 1e-2);
    ///         assert!(($left.value() - $right.value()).abs() < 1e-2);
    ///         assert_eq!($left.alpha(), $right.alpha());
    ///     };
    /// }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// // Black
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(0.0, 0.0, 0.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([0, 0, 0, 255]))
    /// );
    ///
    /// // White
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(0.0, 0.0, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([255, 255, 255, 255]))
    /// );
    ///
    /// // Red
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([255, 0, 0, 255]))
    /// );
    ///
    /// // Green
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(120.0, 1.0, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([0, 255, 0, 255]))
    /// );
    ///
    /// // Blue
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(240.0, 1.0, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([0, 0, 255, 255]))
    /// );
    ///
    /// // Yellow
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(60.0, 1.0, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([255, 255, 0, 255]))
    /// );
    ///
    /// // Cyan
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(180.0, 1.0, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([0, 255, 255, 255]))
    /// );
    ///
    /// // Magenta
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(300.0, 1.0, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([255, 0, 255, 255]))
    /// );
    ///
    /// // Gray (50%)
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(0.0, 0.0, 0.5, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([128, 128, 128, 255]))
    /// );
    ///
    /// // Dark Red (50% value)
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(0.0, 1.0, 0.5, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([128, 0, 0, 255]))
    /// );
    ///
    /// // Light Pink (low saturation red)
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(0.0, 0.25, 1.0, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([255, 191, 191, 255]))
    /// );
    ///
    /// // Olive (yellow-green, 50% brightness)
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(60.0, 1.0, 0.5, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([128, 128, 0, 255]))
    /// );
    ///
    /// // Teal (cyan-green, 50% brightness)
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(180.0, 1.0, 0.5, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([0, 128, 128, 255]))
    /// );
    ///
    /// // Purple (magenta-blue, 50% brightness)
    /// assert_hsv_pixel_eq!(
    ///     HsvPixel::new(300.0, 1.0, 0.5, 255).unwrap(),
    ///     HsvPixel::from(Pixel::new([128, 0, 128, 255]))
    /// );
    ///
    /// # Ok(())
    /// # }
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
        if hue < 0f32 { hue += 360f32 };

        let saturation = if cmax != 0f32 { delta / cmax } else { 0f32 };

        let value = cmax;

        Self { hue, saturation, value, alpha: a }
    }
}
