use thiserror::Error;

use crate::{
    error::IndexResult,
    pipe::{
        FromPipe, IntoPipe, Pipe,
        image::ImagePipe,
        iter::{Elements, Rows},
    },
    pixel::Pixel,
    primitive::{point::Point, size::Size},
};

#[derive(Debug, Error)]
#[error("size does not match pixels size")]
pub struct SizePixelsMismatch;

/// Image representation
pub struct Image {
    size: Size,
    pixels: Box<[Pixel]>,
}

impl Image {
    /// create an image with the given size and buffer
    /// fails if buf's length is not width * length * PIXEL_SIZE
    pub fn new(size: Size, pixels: Box<[Pixel]>) -> Result<Self, SizePixelsMismatch> {
        if pixels.len() != size.area() {
            return Err(SizePixelsMismatch);
        }

        Ok(Image { size, pixels })
    }

    /// create empty image with specified size
    pub fn empty(size: Size) -> Self {
        let width: usize = size.width();
        let height: usize = size.height();

        Self {
            size,
            pixels: vec![Pixel::zero(); width * height].into_boxed_slice(),
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    /// get immutable pixel at selected point
    pub fn pixel(&self, point: Point) -> IndexResult<&Pixel> {
        let index = point.to_index(self.size())?;

        // SAFETY: index from point.to_idx is always valid
        Ok(&self.pixels[index])
    }

    /// get immutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    ///
    /// this should be called only using valid point
    pub unsafe fn pixel_unchecked(&self, point: Point) -> &Pixel {
        let index = unsafe { point.to_index_unchecked(self.size()) };
        &self.pixels[index]
    }

    /// get mutable pixel at selected cordinates
    pub fn pixel_mut(&mut self, point: Point) -> IndexResult<&mut Pixel> {
        let index = point.to_index(self.size())?;

        // SAFETY: index from point.to_idx is always valid
        Ok(&mut self.pixels[index])
    }

    /// get mutable pixel at selected cordinates
    /// without checking bounds
    ///
    /// # Safety
    /// - point must be within image bounds
    ///
    pub unsafe fn pixel_mut_unchecked(&mut self, point: Point) -> &mut Pixel {
        let index = unsafe { point.to_index_unchecked(self.size()) };
        &mut self.pixels[index]
    }

    pub fn rows(&self) -> Rows<ImagePipe<'_>> {
        Rows::new(self.into_pipe())
    }

    pub fn elements(&self) -> Elements<ImagePipe<'_>> {
        Elements::new(self.into_pipe())
    }

    pub fn buffer(&self) -> Box<[u8]> {
        self.pixels
            .iter()
            .flat_map(|px| px.buffer())
            .cloned()
            .collect()
    }
}

impl<'a> IntoPipe for &'a Image {
    type Item = &'a Pixel;

    type IntoPipe = ImagePipe<'a>;

    fn into_pipe(self) -> Self::IntoPipe {
        ImagePipe::new(self)
    }
}

impl FromPipe<Pixel> for Image {
    fn from_pipe<P>(pipe: P) -> Self
    where
        P: IntoPipe<Item = Pixel>,
    {
        let pipe = pipe.into_pipe();
        let mut image = Image::empty(pipe.size());

        image
            .pixels
            .iter_mut()
            .zip(pipe.elements())
            .for_each(|(target, source)| *target = source);
        image
    }
}
