use thiserror::Error;

use crate::{
    error::IndexResult,
    lens::{
        FromLens,
        FromLensPar,
        Lens,
        image::ImageLens,
    },
    pixel::Pixel,
    primitive::{
        point::Point,
        size::Size,
    },
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
    /// fails if pixels' length is not width * length * PIXEL_SIZE
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

        Self { size, pixels: vec![Pixel::zero(); width * height].into_boxed_slice() }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    /// get immutable pixel at selected point
    pub fn pixel(&self, point: Point) -> IndexResult<&Pixel> {
        let index = point.index(self.size())?;

        // SAFETY: index from point.to_idx is always valid
        Ok(&self.pixels[index])
    }

    /// get mutable pixel at selected coordinates
    pub fn pixel_mut(&mut self, point: Point) -> IndexResult<&mut Pixel> {
        let index = point.index(self.size())?;

        // SAFETY: index from point.to_idx is always valid
        Ok(&mut self.pixels[index])
    }

    pub fn buffer(&self) -> Box<[u8]> {
        self.pixels.iter().flat_map(|px| px.buffer()).cloned().collect()
    }

    pub fn lens(&self) -> ImageLens<'_> {
        ImageLens::new(self)
    }
}

impl<T: Into<Pixel>> FromLens<T> for Image {
    fn from_lens<P>(lens: P) -> Self
    where
        P: Lens<Item = T>,
    {
        let size = lens.size();
        let pixels = Box::from_iter(lens.elements().map(Into::into));

        Self::new(size, pixels).unwrap()
    }
}

#[cfg(feature = "parallel")]
impl<T: Into<Pixel> + Send> FromLensPar<T> for Image {
    fn from_lens_par<P>(lens: P) -> Self
    where
        P: Lens<Item = T> + Send + Sync,
        P::Item: Send,
    {
        use std::thread;

        let size = lens.size();
        let cpus = num_cpus::get();
        let chunk_size = (size.area() as f32 / cpus as f32).ceil() as usize;

        let mut image = Image::empty(size);

        let image_chunks = image.pixels.chunks_mut(chunk_size);

        thread::scope(|scope| {
            image_chunks.enumerate().for_each(|(index, chunk)| {
                let lens = &lens;
                scope.spawn(move || {
                    let starting_index = index * chunk_size;
                    chunk.iter_mut().enumerate().for_each(|(index, pixel)| {
                        let point = Point::from_index(starting_index + index, size).unwrap();
                        *pixel = lens.look(point).unwrap().into();
                    });
                });
            });
        });

        image
    }
}
