use crate::{
    error::{BufferLengthMismatchError, BufferLengthMismatchResult},
    pixel::{Pixel, PixelMut, PIXEL_SIZE},
};

/// Iterator over immutable pixels of an image.
/// result of calling pixels() on Image struct
pub struct Pixels<'a> {
    inner: std::slice::Chunks<'a, u8>,
}

impl<'a> Iterator for Pixels<'a> {
    type Item = Pixel<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            // SAFETY: chunks iterator always has PIXEL_SIZE size and
            // buf is guaranted to be divisible by PIXEL_SIZE
            .map(|buf| Pixel::new(buf.try_into().unwrap()))
    }
}

impl<'a> Pixels<'a> {
    pub fn new(buf: &'a [u8]) -> BufferLengthMismatchResult<Self> {
        if buf.len() % PIXEL_SIZE != 0 {
            return Err(BufferLengthMismatchError);
        }

        Ok(Pixels {
            inner: buf.chunks(PIXEL_SIZE),
        })
    }
}

/// Iterator over mutable pixels of an image.
/// result of calling pixels_mut() on Image struct
pub struct PixelsMut<'a> {
    inner: std::slice::ChunksMut<'a, u8>,
}

impl<'a> Iterator for PixelsMut<'a> {
    type Item = PixelMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            // SAFETY: chunks iterator always has PIXEL_SIZE size and
            // buf is guaranted to be divisible by PIXEL_SIZE
            .map(|buf| PixelMut::new(buf.try_into().unwrap()))
    }
}

impl<'a> PixelsMut<'a> {
    pub fn new(buf: &'a mut [u8]) -> BufferLengthMismatchResult<Self> {
        if buf.len() % PIXEL_SIZE != 0 {
            return Err(BufferLengthMismatchError);
        }

        Ok(PixelsMut {
            inner: buf.chunks_mut(PIXEL_SIZE),
        })
    }
}
