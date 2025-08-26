use thiserror::Error;

use crate::{
    error::{IndexResult, OutOfBoundsError},
    iter::{Pixels, PixelsMut, Rows, RowsMut},
    pixel::{PIXEL_SIZE, Pixel, PixelMut},
    primitives::{point::Point, size::Size},
};

#[derive(Debug, Error)]
#[error("size does not match buffer size")]
pub struct SizeBufferMismatch;

pub struct Buffer(Box<[u8]>);

impl Buffer {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_data(&self, index: usize, length: usize) -> IndexResult<&[u8]> {
        if index + length > self.len() {
            return Err(OutOfBoundsError);
        }

        Ok(unsafe { self.get_data_unchecked(index, length) })
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics in debug if index + length <= buffer len.
    ///
    /// # Safety
    /// - index + length have to be within buffer size
    ///
    pub unsafe fn get_data_unchecked(&self, index: usize, length: usize) -> &[u8] {
        debug_assert!(index + length <= self.len(), "buffer out of bounds access");
        &self.0[index..index + length]
    }

    pub fn get_data_mut(&mut self, index: usize, length: usize) -> IndexResult<&mut [u8]> {
        if index + length > self.len() {
            return Err(OutOfBoundsError);
        }

        Ok(unsafe { self.get_data_mut_unchecked(index, length) })
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics in debug if index + length <= buffer len.
    ///
    /// # Safety
    /// - index + length have to be within buffer size
    ///
    pub unsafe fn get_data_mut_unchecked(&mut self, index: usize, length: usize) -> &mut [u8] {
        debug_assert!(index + length <= self.len(), "buffer out of bounds access");
        &mut self.0[index..index + length]
    }
}

impl FromIterator<u8> for Buffer {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Buffer(Box::from_iter(iter))
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

pub(crate) fn buffer_valid_for_size(buffer: &Buffer, size: Size) -> bool {
    let width: usize = size.width();
    let height: usize = size.height();
    buffer.len() == width * height * PIXEL_SIZE
}

/// Image representation
pub struct Image {
    size: Size,
    buffer: Buffer,
}

impl Image {
    /// create an image with the given size and buffer
    /// fails if buf's length is not width * length * PIXEL_SIZE
    pub fn new(size: Size, buffer: Buffer) -> Result<Self, SizeBufferMismatch> {
        if !buffer_valid_for_size(&buffer, size) {
            return Err(SizeBufferMismatch);
        }

        Ok(Image { size, buffer })
    }

    /// create empty image with specified size
    pub fn empty(size: Size) -> Self {
        let width: usize = size.width();
        let height: usize = size.height();

        Self {
            size,
            buffer: Buffer::from_iter(vec![0; width * height * PIXEL_SIZE]),
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    /// get immutable pixel at selected point
    pub fn pixel(&self, point: Point) -> IndexResult<Pixel<'_>> {
        let index = point.to_index(self.size())? * PIXEL_SIZE;

        // SAFETY: index from point.to_idx is always valid
        let data = unsafe { self.buffer.get_data_unchecked(index, PIXEL_SIZE) };

        // SAFETY: data is always PIXEL_SIZE so try_into never fails
        Ok(Pixel::new(data.try_into().unwrap()))
    }

    /// get immutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    ///
    /// this should be called only using valid point
    pub unsafe fn pixel_unchecked(&self, point: Point) -> Pixel<'_> {
        let index = unsafe { point.to_index_unchecked(self.size()) } * PIXEL_SIZE;
        let data = unsafe { self.buffer.get_data_unchecked(index, PIXEL_SIZE) };

        Pixel::new(data.try_into().unwrap())
    }

    /// get mutable pixel at selected cordinates
    pub fn pixel_mut(&mut self, point: Point) -> IndexResult<PixelMut<'_>> {
        let index = point.to_index(self.size())? * PIXEL_SIZE;
        let data = self.buffer.get_data_mut(index, PIXEL_SIZE)?;

        // SAFETY: data is always PIXEL_SIZE so try_into never fails
        Ok(PixelMut::new(data.try_into().unwrap()))
    }

    /// get mutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    /// - point must be within image bounds
    ///
    pub unsafe fn pixel_mut_unchecked(&mut self, point: Point) -> PixelMut<'_> {
        let index = unsafe { point.to_index_unchecked(self.size()) } * PIXEL_SIZE;
        let data = unsafe { self.buffer.get_data_mut_unchecked(index, PIXEL_SIZE) };

        // SAFETY: data is always PIXEL_SIZE so try_into never fails
        PixelMut::new(data.try_into().unwrap())
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn pixels(&self) -> Pixels<'_> {
        // SAFETY: buffer here is always of size N * PIXEL_SIZE
        Pixels::new(self.buffer.as_ref()).unwrap()
    }

    pub fn pixels_mut(&mut self) -> PixelsMut<'_> {
        // SAFETY: buffer here is always of size N * PIXEL_SIZE
        PixelsMut::new(self.buffer.as_mut()).unwrap()
    }

    pub fn rows(&self) -> Rows<'_> {
        // SAFETY: buffer here is always of size width * height * PIXEL_SIZE
        Rows::new(self.buffer.as_ref(), self.size).unwrap()
    }

    pub fn rows_mut(&mut self) -> RowsMut<'_> {
        // SAFETY: buffer here is always of size width * height * PIXEL_SIZE
        RowsMut::new(self.buffer.as_mut(), self.size).unwrap()
    }
}
