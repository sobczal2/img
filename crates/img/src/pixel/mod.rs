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
    #[derive(Clone, Copy, Debug)]
    pub struct ChannelFlags: u8 {
        const R = 0b1000;
        const G = 0b0100;
        const B = 0b0010;
        const A = 0b0001;

        const RGB = Self::R.bits() | Self::G.bits() | Self::B.bits();
        const RGBA = Self::RGB.bits() | Self::A.bits();
    }
}

/// A `struct` representing RGBA pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pixel([u8; PIXEL_SIZE]);

impl Pixel {
    /// Create a [`Pixel`] from given array.
    ///
    /// It uses RGBA layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([255, 128, 64, 200]);
    ///
    /// assert_eq!(pixel.r(), 255);
    /// assert_eq!(pixel.g(), 128);
    /// assert_eq!(pixel.b(), 64);
    /// assert_eq!(pixel.a(), 200);
    /// ```
    pub const fn new(value: [u8; PIXEL_SIZE]) -> Self {
        Self(value)
    }

    /// Create a [`Pixel`] with zeroed channels.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::zero();
    ///
    /// assert_eq!(pixel.r(), 0);
    /// assert_eq!(pixel.g(), 0);
    /// assert_eq!(pixel.b(), 0);
    /// assert_eq!(pixel.a(), 0);
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([100, 0, 0, 0]);
    /// assert_eq!(pixel.r(), 100);
    /// ```
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    /// Get green component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([0, 100, 0, 0]);
    /// assert_eq!(pixel.g(), 100);
    /// ```
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    /// Get blue component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([0, 0, 100, 0]);
    /// assert_eq!(pixel.b(), 100);
    /// ```
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    /// Get alpha component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([0, 0, 0, 100]);
    /// assert_eq!(pixel.a(), 100);
    /// ```
    pub fn a(&self) -> u8 {
        self.0[3]
    }

    /// Set red component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_r(100);
    /// assert_eq!(pixel.r(), 100);
    /// ```
    pub fn set_r(&mut self, value: u8) {
        self.0[0] = value;
    }

    /// Set green component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_g(100);
    /// assert_eq!(pixel.g(), 100);
    /// ```
    pub fn set_g(&mut self, value: u8) {
        self.0[1] = value;
    }

    /// Set blue component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_b(100);
    /// assert_eq!(pixel.b(), 100);
    /// ```
    pub fn set_b(&mut self, value: u8) {
        self.0[2] = value;
    }

    /// Set alpha component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_a(100);
    /// assert_eq!(pixel.a(), 100);
    /// ```
    pub fn set_a(&mut self, value: u8) {
        self.0[3] = value;
    }

    /// Get underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([10, 20, 30, 40]);
    /// assert_eq!(pixel.buffer(), &[10, 20, 30, 40]);
    /// ```
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
    /// pixel.set_with_flags(1, 2, 3, 4, ChannelFlags::R | ChannelFlags::B);
    ///
    /// assert_eq!(1, pixel.r());
    /// assert_eq!(0, pixel.g());
    /// assert_eq!(3, pixel.b());
    /// assert_eq!(0, pixel.a());
    /// ```
    pub fn set_with_flags(&mut self, r: u8, g: u8, b: u8, a: u8, flags: ChannelFlags) {
        if flags.contains(ChannelFlags::R) {
            self.set_r(r);
        }

        if flags.contains(ChannelFlags::G) {
            self.set_g(g);
        }

        if flags.contains(ChannelFlags::B) {
            self.set_b(b);
        }

        if flags.contains(ChannelFlags::A) {
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
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([255, 0, 0, 0]);
    /// assert!((pixel.r_f32() - 1.0).abs() < 1e-6);
    ///
    /// let pixel = Pixel::new([128, 0, 0, 0]);
    /// assert!((pixel.r_f32() - 0.502).abs() < 1e-2);
    /// ```
    fn r_f32(&self) -> f32 {
        self.r() as f32 / 255.0
    }

    /// Get 0-1 normalized green component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([0, 255, 0, 0]);
    /// assert!((pixel.g_f32() - 1.0).abs() < 1e-6);
    /// ```
    fn g_f32(&self) -> f32 {
        self.g() as f32 / 255.0
    }

    /// Get 0-1 normalized blue component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([0, 0, 255, 0]);
    /// assert!((pixel.b_f32() - 1.0).abs() < 1e-6);
    /// ```
    fn b_f32(&self) -> f32 {
        self.b() as f32 / 255.0
    }

    /// Get 0-1 normalized alpha component.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let pixel = Pixel::new([0, 0, 0, 255]);
    /// assert!((pixel.a_f32() - 1.0).abs() < 1e-6);
    /// ```
    fn a_f32(&self) -> f32 {
        self.a() as f32 / 255.0
    }

    /// Set 0-1 normalized red component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_r_f32(0.5);
    /// assert_eq!(pixel.r(), 128);
    ///
    /// // Values are clamped
    /// pixel.set_r_f32(2.0);
    /// assert_eq!(pixel.r(), 255);
    /// pixel.set_r_f32(-1.0);
    /// assert_eq!(pixel.r(), 0);
    /// ```
    fn set_r_f32(&mut self, value: f32) {
        self.set_r((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    /// Set 0-1 normalized green component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_g_f32(0.5);
    /// assert_eq!(pixel.g(), 128);
    /// ```
    fn set_g_f32(&mut self, value: f32) {
        self.set_g((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    /// Set 0-1 normalized blue component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_b_f32(0.5);
    /// assert_eq!(pixel.b(), 128);
    /// ```
    fn set_b_f32(&mut self, value: f32) {
        self.set_b((value * 255.0).round().clamp(0f32, 255f32) as u8);
    }

    /// Set 0-1 normalized alpha component.
    ///
    /// This clamps the result if it is not in 0-1 range.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    ///
    /// let mut pixel = Pixel::zero();
    /// pixel.set_a_f32(0.5);
    /// assert_eq!(pixel.a(), 128);
    /// ```
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
    /// pixel.set_with_flags_f32(0.1, 0.2, 0.3, 0.4, ChannelFlags::R | ChannelFlags::B);
    ///
    /// assert!((pixel.r_f32() - 0.1).abs() < 1e-2);
    /// assert!((pixel.g_f32() - 0.0).abs() < 1e-2);
    /// assert!((pixel.b_f32() - 0.3).abs() < 1e-2);
    /// assert!((pixel.a_f32() - 0.0).abs() < 1e-2);
    /// ```
    fn set_with_flags_f32(&mut self, r: f32, g: f32, b: f32, a: f32, flags: ChannelFlags) {
        if flags.contains(ChannelFlags::R) {
            self.set_r_f32(r);
        }

        if flags.contains(ChannelFlags::G) {
            self.set_g_f32(g);
        }

        if flags.contains(ChannelFlags::B) {
            self.set_b_f32(b);
        }

        if flags.contains(ChannelFlags::A) {
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

#[derive(Debug)]
pub struct ChannelStore<T> {
    r: T,
    g: T,
    b: T,
    a: T,
}

impl<T> ChannelStore<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn() -> T,
    {
        Self::new(f(), f(), f(), f())
    }

    pub fn r(&self) -> &T {
        &self.r
    }

    pub fn g(&self) -> &T {
        &self.g
    }

    pub fn b(&self) -> &T {
        &self.b
    }

    pub fn a(&self) -> &T {
        &self.a
    }

    pub fn r_mut(&mut self) -> &mut T {
        &mut self.r
    }

    pub fn g_mut(&mut self) -> &mut T {
        &mut self.g
    }

    pub fn b_mut(&mut self) -> &mut T {
        &mut self.b
    }

    pub fn a_mut(&mut self) -> &mut T {
        &mut self.a
    }

    pub fn zip_active<U, F, E>(
        &mut self,
        other: &ChannelStore<U>,
        flags: ChannelFlags,
        mut f: F,
    ) -> Result<(), E>
    where
        F: FnMut(&mut T, &U) -> Result<(), E>,
    {
        if flags.contains(ChannelFlags::R) {
            f(&mut self.r, &other.r)?;
        }
        if flags.contains(ChannelFlags::G) {
            f(&mut self.g, &other.g)?;
        }
        if flags.contains(ChannelFlags::B) {
            f(&mut self.b, &other.b)?;
        }
        if flags.contains(ChannelFlags::A) {
            f(&mut self.a, &other.a)?;
        }
        Ok(())
    }

    pub fn map_with<U, R, F>(&self, other: ChannelStore<U>, mut f: F) -> ChannelStore<R>
    where
        F: FnMut(&T, U, ChannelFlags) -> R,
    {
        ChannelStore {
            r: f(&self.r, other.r, ChannelFlags::R),
            g: f(&self.g, other.g, ChannelFlags::G),
            b: f(&self.b, other.b, ChannelFlags::B),
            a: f(&self.a, other.a, ChannelFlags::A),
        }
    }
}

impl From<Pixel> for ChannelStore<u8> {
    fn from(value: Pixel) -> Self {
        ChannelStore { r: value.r(), g: value.g(), b: value.b(), a: value.a() }
    }
}

impl<T> From<ChannelStore<T>> for [T; 4] {
    fn from(value: ChannelStore<T>) -> Self {
        [value.r, value.g, value.b, value.a]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pixel = Pixel::new([10, 20, 30, 40]);
        assert_eq!(pixel.r(), 10);
        assert_eq!(pixel.g(), 20);
        assert_eq!(pixel.b(), 30);
        assert_eq!(pixel.a(), 40);
    }

    #[test]
    fn test_zero() {
        let pixel = Pixel::zero();
        assert_eq!(pixel.r(), 0);
        assert_eq!(pixel.g(), 0);
        assert_eq!(pixel.b(), 0);
        assert_eq!(pixel.a(), 0);
    }

    #[test]
    fn test_default() {
        let pixel = Pixel::default();
        assert_eq!(pixel, Pixel::zero());
    }

    #[test]
    fn test_getters_setters() {
        let mut pixel = Pixel::zero();

        pixel.set_r(100);
        assert_eq!(pixel.r(), 100);

        pixel.set_g(150);
        assert_eq!(pixel.g(), 150);

        pixel.set_b(200);
        assert_eq!(pixel.b(), 200);

        pixel.set_a(250);
        assert_eq!(pixel.a(), 250);
    }

    #[test]
    fn test_buffer() {
        let pixel = Pixel::new([1, 2, 3, 4]);
        assert_eq!(pixel.buffer(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_set_with_flags_all() {
        let mut pixel = Pixel::zero();
        pixel.set_with_flags(10, 20, 30, 40, ChannelFlags::RGBA);
        assert_eq!(pixel, Pixel::new([10, 20, 30, 40]));
    }

    #[test]
    fn test_set_with_flags_partial() {
        let mut pixel = Pixel::new([255, 255, 255, 255]);
        pixel.set_with_flags(0, 0, 0, 0, ChannelFlags::R | ChannelFlags::A);
        assert_eq!(pixel, Pixel::new([0, 255, 255, 0]));
    }

    #[test]
    fn test_f32_getters() {
        let pixel = Pixel::new([0, 128, 255, 64]);
        assert!((pixel.r_f32() - 0.0).abs() < 1e-6);
        assert!((pixel.g_f32() - 0.502).abs() < 1e-2);
        assert!((pixel.b_f32() - 1.0).abs() < 1e-6);
        assert!((pixel.a_f32() - 0.251).abs() < 1e-2);
    }

    #[test]
    fn test_f32_setters() {
        let mut pixel = Pixel::zero();
        pixel.set_r_f32(0.0);
        pixel.set_g_f32(0.5);
        pixel.set_b_f32(1.0);
        pixel.set_a_f32(0.25);
        assert_eq!(pixel.r(), 0);
        assert_eq!(pixel.g(), 128);
        assert_eq!(pixel.b(), 255);
        assert_eq!(pixel.a(), 64);
    }

    #[test]
    fn test_f32_setters_clamping() {
        let mut pixel = Pixel::zero();

        // Test upper clamping
        pixel.set_r_f32(2.0);
        assert_eq!(pixel.r(), 255);

        // Test lower clamping
        pixel.set_g_f32(-1.0);
        assert_eq!(pixel.g(), 0);
    }

    #[test]
    fn test_equality() {
        let p1 = Pixel::new([10, 20, 30, 40]);
        let p2 = Pixel::new([10, 20, 30, 40]);
        let p3 = Pixel::new([10, 20, 30, 41]);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_clone() {
        let p1 = Pixel::new([10, 20, 30, 40]);
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_as_ref() {
        let pixel = Pixel::new([1, 2, 3, 4]);
        let pixel_ref: &Pixel = pixel.as_ref();
        assert_eq!(pixel_ref.r(), 1);
    }

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
