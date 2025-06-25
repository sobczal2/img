#![allow(unused)]
// pixel size of an image in bytes
pub const PIXEL_SIZE: usize = 4;

/// A immutable view of RGBA pixel in some image
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel<'a>(&'a [u8; PIXEL_SIZE]);

/// A mutable view of RGBA pixel in some image
#[derive(Debug, PartialEq, Eq)]
pub struct PixelMut<'a>(&'a mut [u8; PIXEL_SIZE]);

impl<'a> Pixel<'a> {
    pub fn new(value: &'a [u8; PIXEL_SIZE]) -> Self {
        Pixel(value)
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
}

impl<'a> PixelMut<'a> {
    pub fn new(value: &'a mut [u8; PIXEL_SIZE]) -> Self {
        PixelMut(value)
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

    pub fn copy_from_pixel<'p>(&mut self, px: impl Into<Pixel<'p>>) {
        let px: Pixel<'p> = px.into();
        self.0.copy_from_slice(px.0);
    }
}

impl<'a> From<PixelMut<'a>> for Pixel<'a> {
    fn from(value: PixelMut<'a>) -> Self {
        Pixel(value.0)
    }
}

pub trait ReadPixelRgbaf32 {
    fn r_f32(&self) -> f32;
    fn g_f32(&self) -> f32;
    fn b_f32(&self) -> f32;
    fn a_f32(&self) -> f32;
}

impl ReadPixelRgbaf32 for Pixel<'_> {
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
}

impl ReadPixelRgbaf32 for PixelMut<'_> {
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
}

pub trait WritePixelRgbaf32 {
    fn set_r_f32(&mut self, value: f32);
    fn set_g_f32(&mut self, value: f32);
    fn set_b_f32(&mut self, value: f32);
    fn set_a_f32(&mut self, value: f32);
}

impl WritePixelRgbaf32 for PixelMut<'_> {
    fn set_r_f32(&mut self, value: f32) {
        self.set_r((value * 255.0).clamp(0f32, 255f32) as u8);
    }

    fn set_g_f32(&mut self, value: f32) {
        self.set_g((value * 255.0).clamp(0f32, 255f32) as u8);
    }

    fn set_b_f32(&mut self, value: f32) {
        self.set_b((value * 255.0).clamp(0f32, 255f32) as u8);
    }

    fn set_a_f32(&mut self, value: f32) {
        self.set_a((value * 255.0).clamp(0f32, 255f32) as u8);
    }
}
