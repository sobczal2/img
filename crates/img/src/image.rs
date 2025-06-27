use thiserror::Error;

use crate::{
    error::{IndexResult, OutOfBoundsError},
    iter::{Pixels, PixelsMut, Rows, RowsMut},
    math::xy_to_idx,
    pixel::{Pixel, PixelMut, PIXEL_SIZE},
};

#[derive(Debug, Error)]
#[error("size does not match buffer size")]
pub struct SizeBufferMismatch;

/// Image representation
pub struct Image {
    size: (usize, usize),
    buf: Box<[u8]>,
}

impl Image {
    /// create an image with the given size and buffer
    /// fails if buf's length is not width * length * PIXEL_SIZE
    pub fn new(size: (usize, usize), buf: Box<[u8]>) -> Result<Self, SizeBufferMismatch> {
        if buf.len() != size.0 * size.1 * PIXEL_SIZE {
            return Err(SizeBufferMismatch);
        }

        Ok(Image { size, buf })
    }

    /// create empty image with specified size
    pub fn empty(size: (usize, usize)) -> Self {
        Self {
            size,
            buf: vec![0; size.0 * size.1 * PIXEL_SIZE].into_boxed_slice(),
        }
    }

    /// get size of the image in pixels
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    /// get immutable pixel at selected cordinates
    pub fn pixel(&self, xy: (usize, usize)) -> IndexResult<Pixel<'_>> {
        let idx = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        if idx > self.buf.len() {
            return Err(OutOfBoundsError);
        }

        let buf = &self.buf[idx..idx + PIXEL_SIZE];
        Ok(Pixel::new(buf.try_into().unwrap()))
    }

    /// get immutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    ///
    /// this should be called only using valid x and y
    pub unsafe fn pixel_unchecked(&self, xy: (usize, usize)) -> Pixel<'_> {
        let idx = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        let buf = &self.buf[idx..idx + PIXEL_SIZE];
        Pixel::new(buf.try_into().unwrap())
    }

    /// get mutable pixel at selected cordinates
    pub fn pixel_mut(&mut self, xy: (usize, usize)) -> IndexResult<PixelMut<'_>> {
        let idx = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        if idx > self.buf.len() {
            return Err(OutOfBoundsError);
        }

        let buf = &mut self.buf[idx..idx + PIXEL_SIZE];
        Ok(PixelMut::new(buf.try_into().unwrap()))
    }

    /// get mutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    ///
    /// this should be called only using valid x and y
    pub unsafe fn pixel_mut_unchecked(&mut self, xy: (usize, usize)) -> PixelMut<'_> {
        let idx = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        let buf = &mut self.buf[idx..idx + PIXEL_SIZE];
        PixelMut::new(buf.try_into().unwrap())
    }

    /// get readonly underlying buffer
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }

    pub fn pixels(&self) -> Pixels {
        // SAFETY: buffer here is always of size N * PIXEL_SIZE
        Pixels::new(&self.buf).unwrap()
    }

    pub fn pixels_mut(&mut self) -> PixelsMut {
        // SAFETY: buffer here is always of size N * PIXEL_SIZE
        PixelsMut::new(&mut self.buf).unwrap()
    }

    pub fn rows(&self) -> Rows {
        // SAFETY: buffer here is always of size width * height * PIXEL_SIZE
        Rows::new(&self.buf, self.size).unwrap()
    }

    pub fn rows_mut(&mut self) -> RowsMut {
        // SAFETY: buffer here is always of size width * height * PIXEL_SIZE
        RowsMut::new(&mut self.buf, self.size).unwrap()
    }
}
