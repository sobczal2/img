use crate::{
    error::{BufferLengthMismatchError, BufferLengthMismatchResult},
    pixel::{Pixel, PixelMut, PIXEL_SIZE}, primitives::size::Size,
};

/// Iterator over immutable pixels of an image.
/// result of calling pixels() on Image struct
#[derive(Debug, Clone)]
pub struct Pixels<'a> {
    inner: std::slice::ChunksExact<'a, u8>,
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
    pub fn new(buffer: &'a [u8]) -> BufferLengthMismatchResult<Self> {
        if buffer.len() % PIXEL_SIZE != 0 {
            return Err(BufferLengthMismatchError);
        }

        Ok(Pixels {
            inner: buffer.chunks_exact(PIXEL_SIZE),
        })
    }
}

/// Iterator over mutable pixels of an image.
/// result of calling pixels_mut() on Image struct
pub struct PixelsMut<'a> {
    inner: std::slice::ChunksExactMut<'a, u8>,
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
    pub fn new(buffer: &'a mut [u8]) -> BufferLengthMismatchResult<Self> {
        if buffer.len() % PIXEL_SIZE != 0 {
            return Err(BufferLengthMismatchError);
        }

        Ok(PixelsMut {
            inner: buffer.chunks_exact_mut(PIXEL_SIZE),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Row<'a> {
    pixels: Pixels<'a>,
    position: usize,
}

impl<'a> Iterator for Row<'a> {
    type Item = (usize, Pixel<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pixel) = self.pixels.next() {
            let position = self.position;
            self.position += 1;
            return Some((position, pixel));
        }

        None
    }
}

impl<'a> Row<'a> {
    pub fn new(buffer: &'a [u8]) -> BufferLengthMismatchResult<Self> {
        Ok(Row {
            pixels: Pixels::new(buffer)?,
            position: 0,
        })
    }
}

pub struct RowMut<'a> {
    pixels: PixelsMut<'a>,
    position: usize,
}

impl<'a> Iterator for RowMut<'a> {
    type Item = (usize, PixelMut<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pixel) = self.pixels.next() {
            let position = self.position;
            self.position += 1;
            return Some((position, pixel));
        }

        None
    }
}

impl<'a> RowMut<'a> {
    pub fn new(buffer: &'a mut [u8]) -> BufferLengthMismatchResult<Self> {
        Ok(RowMut {
            pixels: PixelsMut::new(buffer)?,
            position: 0,
        })
    }
}

pub struct Rows<'a> {
    rows: std::slice::ChunksExact<'a, u8>,
    position: usize,
}

impl<'a> Iterator for Rows<'a> {
    type Item = (usize, Row<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(buf) = self.rows.next() {
            let position = self.position;
            self.position += 1;
            // SAFETY: row is guaranted to contain width * PIXEL_SIZE bytes
            return Some((position, Row::new(buf).unwrap()));
        }

        None
    }
}

impl<'a> Rows<'a> {
    pub fn new(buffer: &'a [u8], size: Size) -> BufferLengthMismatchResult<Self> {
        let width: usize = size.width().into();
        let height: usize = size.height().into();
        if buffer.len() != width * height * PIXEL_SIZE {
            return Err(BufferLengthMismatchError);
        }

        Ok(Rows {
            rows: buffer.chunks_exact(width * PIXEL_SIZE),
            position: 0,
        })
    }
}

pub struct RowsMut<'a> {
    rows: std::slice::ChunksExactMut<'a, u8>,
    position: usize,
}

impl<'a> Iterator for RowsMut<'a> {
    type Item = (usize, RowMut<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(buf) = self.rows.next() {
            let position = self.position;
            self.position += 1;
            // SAFETY: row is guaranted to contain width * PIXEL_SIZE bytes
            return Some((position, RowMut::new(buf).unwrap()));
        }

        None
    }
}

impl<'a> RowsMut<'a> {
    pub fn new(buffer: &'a mut [u8], size: Size) -> BufferLengthMismatchResult<Self> {
        let width: usize = size.width().into();
        let height: usize = size.height().into();
        if buffer.len() != width * height * PIXEL_SIZE {
            return Err(BufferLengthMismatchError);
        }

        Ok(RowsMut {
            rows: buffer.chunks_exact_mut(width * PIXEL_SIZE),
            position: 0,
        })
    }
}
