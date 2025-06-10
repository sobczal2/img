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

    /// get red value mutably
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.0[0]
    }

    /// get green value mutably
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.0[1]
    }

    /// get blue value mutably
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.0[2]
    }

    /// get alpha value mutably
    pub fn a_mut(&mut self) -> &mut u8 {
        &mut self.0[3]
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
