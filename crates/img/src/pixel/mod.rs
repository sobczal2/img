use bitflags::bitflags;
use rand::Rng;

use crate::pixel::hsv::HsvPixel;

pub mod hsv;

/// Pixel size of an image in bytes
///
/// Currently image is always represented as RGBA image with 8 bits per pixel.
/// This constant is left here in case this changes in the future.
pub const PIXEL_SIZE: usize = 4;

bitflags! {
    /// A `struct` representing image channels.
    ///
    /// Some operations suppport this as a parameter to specify which channel should be
    /// affected.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ChannelFlags: u8 {
        const RED = 0b1000;
        const GREEN = 0b0100;
        const BLUE = 0b0010;
        const ALPHA = 0b0001;

        const RGB = Self::RED.bits() | Self::GREEN.bits() | Self::BLUE.bits();
        const RGBA = Self::RGB.bits() | Self::ALPHA.bits();
    }
}

/// A `struct` representing RGBA pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pixel([u8; PIXEL_SIZE]);

impl Pixel {
    /// Create a [`Pixel`] from given array.
    ///
    /// It uses RGBA layout.
    pub const fn new(value: [u8; PIXEL_SIZE]) -> Self {
        Self(value)
    }

    /// Create a [`Pixel`] with zeroed channels.
    pub const fn zero() -> Self {
        Self([0; PIXEL_SIZE])
    }

    /// Create a [`Pixel`] with random channel values.
    pub fn random<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        Pixel(rng.random())
    }

    /// Get red component.
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    /// Get green component.
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    /// Get blue component.
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    /// Get alpha component.
    pub fn a(&self) -> u8 {
        self.0[3]
    }

    /// Set red component.
    pub fn set_r(&mut self, value: u8) {
        self.0[0] = value;
    }

    /// Set green component.
    pub fn set_g(&mut self, value: u8) {
        self.0[1] = value;
    }

    /// Set blue component.
    pub fn set_b(&mut self, value: u8) {
        self.0[2] = value;
    }

    /// Set alpha component.
    pub fn set_a(&mut self, value: u8) {
        self.0[3] = value;
    }

    /// Get underlying array.
    pub fn buffer(&self) -> &[u8; PIXEL_SIZE] {
        &self.0
    }

    /// Set [`Pixel`] values ignoring channels not specified in `flags`.
    ///
    /// # Examples
    /// ```
    /// use img::prelude::*;
    /// let mut pixel = Pixel::zero();
    ///
    /// pixel.set_with_flags(1, 2, 3, 4, ChannelFlags::RED | ChannelFlags::BLUE);
    ///
    /// assert_eq!(1, pixel.r());
    /// assert_eq!(0, pixel.g());
    /// assert_eq!(3, pixel.b());
    /// assert_eq!(0, pixel.a());
    /// ```
    pub fn set_with_flags(&mut self, r: u8, g: u8, b: u8, a: u8, flags: ChannelFlags) {
        if flags.contains(ChannelFlags::RED) {
            self.set_r(r);
        }

        if flags.contains(ChannelFlags::GREEN) {
            self.set_g(g);
        }

        if flags.contains(ChannelFlags::BLUE) {
            self.set_b(b);
        }

        if flags.contains(ChannelFlags::ALPHA) {
            self.set_a(a);
        }
    }
}

pub trait PixelRgbaf32 {
    /// Get 0-1 normalized red component.
    fn r_f32(&self) -> f32;

    /// Get 0-1 normalized green component.
    fn g_f32(&self) -> f32;

    /// Get 0-1 normalized blue component.
    fn b_f32(&self) -> f32;

    /// Get 0-1 normalized alpha component.
    fn a_f32(&self) -> f32;

    /// Set 0-1 normalized red component.
    fn set_r_f32(&mut self, value: f32);

    /// Set 0-1 normalized green component.
    fn set_g_f32(&mut self, value: f32);

    /// Set 0-1 normalized blue component.
    fn set_b_f32(&mut self, value: f32);

    /// Set 0-1 normalized alpha component.
    fn set_a_f32(&mut self, value: f32);

    /// Set [`Pixel`] values ignoring channels not specified in `flags`.
    fn set_with_flags_f32(&mut self, r: f32, g: f32, b: f32, a: f32, flags: ChannelFlags);
}

impl PixelRgbaf32 for Pixel {
    /// Get 0-1 normalized red component.
    fn r_f32(&self) -> f32 {
        self.r() as f32 / 255.0
    }

    /// Get 0-1 normalized green component.
    fn g_f32(&self) -> f32 {
        self.g() as f32 / 255.0
    }

    /// Get 0-1 normalized blue component.
    fn b_f32(&self) -> f32 {
        self.b() as f32 / 255.0
    }

    /// Get 0-1 normalized alpha component.
    fn a_f32(&self) -> f32 {
        self.a() as f32 / 255.0
    }

    /// Set 0-1 normalized red component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    fn set_r_f32(&mut self, value: f32) {
        self.set_r((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    /// Set 0-1 normalized green component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    fn set_g_f32(&mut self, value: f32) {
        self.set_g((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    /// Set 0-1 normalized alpha component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    fn set_b_f32(&mut self, value: f32) {
        self.set_b((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    /// Set 0-1 normalized alpha component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    fn set_a_f32(&mut self, value: f32) {
        self.set_a((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    /// Set [`Pixel`] values ignoring channels not specified in `flags`.
    ///
    /// # Examples
    /// ```
    /// use img::prelude::*;
    /// let mut pixel = Pixel::zero();
    ///
    /// pixel.set_with_flags_f32(0.1, 0.2, 0.3, 0.4, ChannelFlags::RED | ChannelFlags::BLUE);
    ///
    /// assert!((pixel.r_f32() - 0.1).abs() < 1e-2);
    /// assert!((pixel.g_f32() - 0.0).abs() < 1e-2);
    /// assert!((pixel.b_f32() - 0.3).abs() < 1e-2);
    /// assert!((pixel.a_f32() - 0.0).abs() < 1e-2);
    /// ```
    fn set_with_flags_f32(&mut self, r: f32, g: f32, b: f32, a: f32, flags: ChannelFlags) {
        if flags.contains(ChannelFlags::RED) {
            self.set_r_f32(r);
        }

        if flags.contains(ChannelFlags::GREEN) {
            self.set_g_f32(g);
        }

        if flags.contains(ChannelFlags::BLUE) {
            self.set_b_f32(b);
        }

        if flags.contains(ChannelFlags::ALPHA) {
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
    /// use img::pixel::{
    ///     Pixel,
    ///     hsv::HsvPixel,
    /// };
    ///
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(0.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([255, 0, 0, 255])
    /// );
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(120.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([0, 255, 0, 255])
    /// );
    /// assert_eq!(
    ///     Pixel::from(HsvPixel::new(240.0, 1.0, 1.0, 255).unwrap()),
    ///     Pixel::new([0, 0, 255, 255])
    /// );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_from_hsv_pixel() {
        let cases = vec![
            // hue, sat, val, alpha, expected [r, g, b, a]
            (0.0, 0.0, 0.0, 255, [0, 0, 0, 255]),
            (0.0, 0.0, 0.0, 128, [0, 0, 0, 128]),
            (0.0, 0.0, 0.0, 0, [0, 0, 0, 0]),
            (0.0, 0.0, 1.0, 255, [255, 255, 255, 255]),
            (0.0, 1.0, 1.0, 255, [255, 0, 0, 255]),
            (120.0, 1.0, 1.0, 255, [0, 255, 0, 255]),
            (240.0, 1.0, 1.0, 255, [0, 0, 255, 255]),
            (60.0, 1.0, 1.0, 255, [255, 255, 0, 255]),
            (180.0, 1.0, 1.0, 255, [0, 255, 255, 255]),
            (300.0, 1.0, 1.0, 255, [255, 0, 255, 255]),
            (0.0, 0.0, 0.5, 255, [128, 128, 128, 255]),
            (0.0, 1.0, 0.5, 255, [128, 0, 0, 255]),
            (0.0, 0.25, 1.0, 255, [255, 191, 191, 255]),
            (60.0, 1.0, 0.5, 255, [128, 128, 0, 255]),
            (180.0, 1.0, 0.5, 255, [0, 128, 128, 255]),
            (300.0, 1.0, 0.5, 255, [128, 0, 128, 255]),
        ];

        for (h, s, v, a, expected) in cases {
            let hsv = HsvPixel::new(h, s, v, a).unwrap();
            let pixel = Pixel::from(hsv);
            assert_eq!(pixel, Pixel::new(expected), "Failed for HSV({}, {}, {}, {})", h, s, v, a);
        }
    }
}
