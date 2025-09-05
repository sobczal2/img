use bitflags::bitflags;

use crate::pixel::hsv::HsvPixel;

pub mod hsv;

// pixel size of an image in bytes
pub const PIXEL_SIZE: usize = 4;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct PixelFlags: u8 {
        const RED = 0b1000;
        const GREEN = 0b0100;
        const BLUE = 0b0010;
        const ALPHA = 0b0001;

        const RGB = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits();
        const RGBA = Self::RGB.bits() | Self::ALPHA.bits();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel([u8; PIXEL_SIZE]);

impl Pixel {
    pub const fn new(value: [u8; PIXEL_SIZE]) -> Self {
        Self(value)
    }

    pub const fn zero() -> Self {
        Self([0; PIXEL_SIZE])
    }

    /// get red value
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    /// get green value
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    /// get blue value
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    /// get alpha value
    pub fn a(&self) -> u8 {
        self.0[3]
    }

    /// set red value
    pub fn set_r(&mut self, value: u8) {
        self.0[0] = value;
    }

    /// set green value
    pub fn set_g(&mut self, value: u8) {
        self.0[1] = value;
    }

    /// set blue value
    pub fn set_b(&mut self, value: u8) {
        self.0[2] = value;
    }

    /// set alpha value
    pub fn set_a(&mut self, value: u8) {
        self.0[3] = value;
    }

    pub fn buffer(&self) -> &[u8] {
        &self.0
    }

    pub fn set_with_flags(&mut self, r: u8, g: u8, b: u8, a: u8, flags: PixelFlags) {
        if flags.contains(PixelFlags::RED) {
            self.set_r(r);
        }

        if flags.contains(PixelFlags::GREEN) {
            self.set_g(g);
        }

        if flags.contains(PixelFlags::BLUE) {
            self.set_b(b);
        }

        if flags.contains(PixelFlags::ALPHA) {
            self.set_a(a);
        }
    }
}

pub trait PixelRgbaf32 {
    fn r_f32(&self) -> f32;
    fn g_f32(&self) -> f32;
    fn b_f32(&self) -> f32;
    fn a_f32(&self) -> f32;
    fn set_r_f32(&mut self, value: f32);
    fn set_g_f32(&mut self, value: f32);
    fn set_b_f32(&mut self, value: f32);
    fn set_a_f32(&mut self, value: f32);
    fn set_with_flags_f32(&mut self, r: f32, g: f32, b: f32, a: f32, flags: PixelFlags);
}

impl PixelRgbaf32 for Pixel {
    fn r_f32(&self) -> f32 {
        self.r() as f32 / 255.0
    }

    fn g_f32(&self) -> f32 {
        self.g() as f32 / 255.0
    }

    fn b_f32(&self) -> f32 {
        self.b() as f32 / 255.0
    }

    fn a_f32(&self) -> f32 {
        self.a() as f32 / 255.0
    }
    fn set_r_f32(&mut self, value: f32) {
        self.set_r((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    fn set_g_f32(&mut self, value: f32) {
        self.set_g((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    fn set_b_f32(&mut self, value: f32) {
        self.set_b((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    fn set_a_f32(&mut self, value: f32) {
        self.set_a((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    fn set_with_flags_f32(&mut self, r: f32, g: f32, b: f32, a: f32, flags: PixelFlags) {
        if flags.contains(PixelFlags::RED) {
            self.set_r_f32(r);
        }

        if flags.contains(PixelFlags::GREEN) {
            self.set_g_f32(g);
        }

        if flags.contains(PixelFlags::BLUE) {
            self.set_b_f32(b);
        }

        if flags.contains(PixelFlags::ALPHA) {
            self.set_a_f32(a);
        }
    }
}

impl AsRef<Pixel> for Pixel {
    fn as_ref(&self) -> &Pixel {
        self
    }
}

impl From<HsvPixel> for Pixel {

    /// Convert `HsvPixel` to `Pixel`. This effectively converts HSV color space to RGB.
    ///
    /// # Examples
    /// 
    /// ```
    /// use img::pixel::{Pixel, hsv::HsvPixel};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// // Black
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(0.0, 0.0, 0.0, 255).unwrap()),
    ///     Pixel::new([0, 0, 0, 255])
    /// );
    ///
    /// // White
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(0.0, 0.0, 1.0, 255).unwrap()),
    ///     Pixel::new([255, 255, 255, 255])
    /// );
    ///
    /// // Red
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([255, 0, 0, 255])
    /// );
    ///
    /// // Green
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(120.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([0, 255, 0, 255])
    /// );
    ///
    /// // Blue
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(240.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([0, 0, 255, 255])
    /// );
    ///
    /// // Yellow
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(60.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([255, 255, 0, 255])
    /// );
    ///
    /// // Cyan
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(180.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([0, 255, 255, 255])
    /// );
    ///
    /// // Magenta
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(300.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([255, 0, 255, 255])
    /// );
    ///
    /// // Gray (50%)
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(0.0, 0.0, 0.5, 255).unwrap()),
    ///     Pixel::new([128, 128, 128, 255])
    /// );
    ///
    /// // Dark Red (50% value)
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(0.0, 1.0, 0.5, 255).unwrap()),
    ///     Pixel::new([128, 0, 0, 255])
    /// );
    ///
    /// // Light Pink (low saturation red)
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(0.0, 0.25, 1.0, 255).unwrap()),
    ///     Pixel::new([255, 191, 191, 255])
    /// );
    ///
    /// // Olive (yellow-green, 50% brightness)
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(60.0, 1.0, 0.5, 255).unwrap()),
    ///     Pixel::new([128, 128, 0, 255])
    /// );
    ///
    /// // Teal (cyan-green, 50% brightness)
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(180.0, 1.0, 0.5, 255).unwrap()),
    ///     Pixel::new([0, 128, 128, 255])
    /// );
    ///
    /// // Purple (magenta-blue, 50% brightness)
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(300.0, 1.0, 0.5, 255).unwrap()),
    ///     Pixel::new([128, 0, 128, 255])
    /// );
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn from(value: HsvPixel) -> Self {
        let c = value.value() * value.saturation();

        let h = (value.hue() / 60f32) as i8;

        let x = c * (1 - (h % 2 - 1).abs()) as f32;

        let (r1, g1, b1) = match h {
            0 => (c, x, 0f32),
            1 => (x, c, 0f32),
            2 => (0f32, c, x),
            3 => (0f32, x, c),
            4 => (x, 0f32, c),
            5 => (c, 0f32, x),
            _ => unreachable!(),
        };

        let m = value.value() - c;

        let mut pixel = Pixel::zero();
        pixel.set_r_f32(r1 + m);
        pixel.set_g_f32(g1 + m);
        pixel.set_b_f32(b1 + m);
        pixel.set_a(value.alpha());

        pixel
    }
}
