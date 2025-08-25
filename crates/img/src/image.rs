use std::num::NonZeroUsize;

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

#[derive(Debug, Error)]
pub enum SizeCreationError {
    #[error("width is zero")]
    WidthZero,
    #[error("height is zero")]
    HeightZero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(NonZeroUsize, NonZeroUsize);

impl Size {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self(width, height)
    }

    pub fn from_usize(width: usize, height: usize) -> Result<Self, SizeCreationError> {
        let width: NonZeroUsize = width.try_into().map_err(|_| SizeCreationError::WidthZero)?;
        let height: NonZeroUsize = height
            .try_into()
            .map_err(|_| SizeCreationError::WidthZero)?;

        Ok(Size(width, height))
    }

    pub fn width(&self) -> NonZeroUsize {
        self.0
    }

    pub fn height(&self) -> NonZeroUsize {
        self.0
    }
}

pub struct Buffer(Box<[u8]>);

impl Buffer {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_data<'a>(&'a self, index: usize, length: usize) -> IndexResult<&'a [u8]> {
        if index + length >= self.len() {
            return Err(OutOfBoundsError);
        }

        Ok(&self.0[index..index + length])
    }

    pub unsafe fn get_data_unchecked<'a>(&'a self, index: usize, length: usize) -> &'a [u8] {
        debug_assert!(index + length >= self.len(), "buffer out of bounds access");
        &self.0[index..index + length]
    }

    pub fn get_data_mut<'a>(
        &'a mut self,
        index: usize,
        length: usize,
    ) -> IndexResult<&'a mut [u8]> {
        if index + length >= self.len() {
            return Err(OutOfBoundsError);
        }

        Ok(&mut self.0[index..index + length])
    }

    pub fn get_data_mut_unchecked<'a>(&'a mut self, index: usize, length: usize) -> &'a mut [u8] {
        debug_assert!(index + length >= self.len(), "buffer out of bounds access");
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
    let width: usize = size.width().into();
    let height: usize = size.height().into();
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
        let width: usize = size.width().into();
        let height: usize = size.height().into();

        Self {
            size,
            buffer: Buffer::from_iter(vec![0; width * height * PIXEL_SIZE]),
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    /// get immutable pixel at selected cordinates
    pub fn pixel(&self, xy: (usize, usize)) -> IndexResult<Pixel<'_>> {
        let index = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        let data = self.buffer.get_data(index, PIXEL_SIZE)?;

        // SAFETY: data is always PIXEL_SIZE so try_into never fails
        Ok(Pixel::new(data.try_into().unwrap()))
    }

    /// get immutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    ///
    /// this should be called only using valid x and y
    pub unsafe fn pixel_unchecked(&self, xy: (usize, usize)) -> Pixel<'_> {
        let index = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        let data = unsafe { self.buffer.get_data_unchecked(index, PIXEL_SIZE) };

        // SAFETY: data is always PIXEL_SIZE so try_into never fails
        Pixel::new(data.try_into().unwrap())
    }

    /// get mutable pixel at selected cordinates
    pub fn pixel_mut(&mut self, xy: (usize, usize)) -> IndexResult<PixelMut<'_>> {
        let index = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        let data = self.buffer.get_data_mut(index, PIXEL_SIZE)?;

        // SAFETY: data is always PIXEL_SIZE so try_into never fails
        Ok(PixelMut::new(data.try_into().unwrap()))
    }

    /// get mutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    ///
    /// this should be called only using valid x and y
    pub unsafe fn pixel_mut_unchecked(&mut self, xy: (usize, usize)) -> PixelMut<'_> {
        let index = xy_to_idx(xy, self.size.0) * PIXEL_SIZE;
        let data = self.buffer.get_data_mut_unchecked(index, PIXEL_SIZE);

        // SAFETY: data is always PIXEL_SIZE so try_into never fails
        PixelMut::new(data.try_into().unwrap())
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn pixels(&self) -> Pixels {
        // SAFETY: buffer here is always of size N * PIXEL_SIZE
        Pixels::new(self.buffer.as_ref()).unwrap()
    }

    pub fn pixels_mut(&mut self) -> PixelsMut {
        // SAFETY: buffer here is always of size N * PIXEL_SIZE
        PixelsMut::new(self.buffer.as_mut()).unwrap()
    }

    pub fn rows(&self) -> Rows {
        // SAFETY: buffer here is always of size width * height * PIXEL_SIZE
        Rows::new(self.buffer.as_ref(), self.size).unwrap()
    }

    pub fn rows_mut(&mut self) -> RowsMut {
        // SAFETY: buffer here is always of size width * height * PIXEL_SIZE
        RowsMut::new(self.buffer.as_mut(), self.size).unwrap()
    }
}
