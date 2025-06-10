use crate::{
    error::{BufferLengthMismatchError, BufferLengthMismatchResult},
    pixel::{Pixel, PixelMut, PIXEL_SIZE},
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
    pub fn new(buf: &'a [u8]) -> BufferLengthMismatchResult<Self> {
        if buf.len() % PIXEL_SIZE != 0 {
            return Err(BufferLengthMismatchError);
        }

        Ok(Pixels {
            inner: buf.chunks_exact(PIXEL_SIZE),
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
    pub fn new(buf: &'a mut [u8]) -> BufferLengthMismatchResult<Self> {
        if buf.len() % PIXEL_SIZE != 0 {
            return Err(BufferLengthMismatchError);
        }

        Ok(PixelsMut {
            inner: buf.chunks_exact_mut(PIXEL_SIZE),
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
    pub fn new(buf: &'a [u8]) -> BufferLengthMismatchResult<Self> {
        Ok(Row {
            pixels: Pixels::new(buf)?,
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
    pub fn new(buf: &'a mut [u8]) -> BufferLengthMismatchResult<Self> {
        Ok(RowMut {
            pixels: PixelsMut::new(buf)?,
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
    pub fn new(buf: &'a [u8], size: (usize, usize)) -> BufferLengthMismatchResult<Self> {
        if buf.len() != size.0 * size.1 * PIXEL_SIZE {
            return Err(BufferLengthMismatchError);
        }

        Ok(Rows {
            rows: buf.chunks_exact(size.0 * PIXEL_SIZE),
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
    pub fn new(buf: &'a mut [u8], size: (usize, usize)) -> BufferLengthMismatchResult<Self> {
        if buf.len() != size.0 * size.1 * PIXEL_SIZE {
            return Err(BufferLengthMismatchError);
        }

        Ok(RowsMut {
            rows: buf.chunks_exact_mut(size.0 * PIXEL_SIZE),
            position: 0,
        })
    }
}
