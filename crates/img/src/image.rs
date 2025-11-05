use std::iter::from_fn;
#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use rand::Rng;
use thiserror::Error;

use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::IndexResult,
    lens::{
        FromLens,
        FromLensPar,
        Lens,
        image::ImageLens,
    },
    pixel::Pixel,
};

#[derive(Debug, Error, PartialEq, Eq)]
#[error("size does not match pixels size")]
pub struct SizePixelsMismatch;

/// A `struct` representing in-memory image.
#[derive(Debug, Clone)]
pub struct Image {
    size: Size,
    pixels: Box<[Pixel]>,
}

impl Image {
    /// Create an [`Image`] with the given size and buffer.
    ///
    /// Returns [`Image`] if `pixels` length is equal to `size.area()`, [`SizePixelsMismatch`]
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     image::SizePixelsMismatch,
    ///     prelude::*,
    /// };
    /// use std::iter::from_fn;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let pixels: Box<[_]> = from_fn(|| Some(Pixel::zero())).take(4).collect();
    /// let image = Image::new(Size::from_usize(2, 2).unwrap(), pixels.clone())?;
    ///
    /// let mismatch = Image::new(Size::from_usize(2, 3).unwrap(), pixels);
    /// assert_eq!(mismatch.unwrap_err(), SizePixelsMismatch);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(size: Size, pixels: Box<[Pixel]>) -> Result<Self, SizePixelsMismatch> {
        if pixels.len() != size.area() {
            return Err(SizePixelsMismatch);
        }

        Ok(Image { size, pixels })
    }

    /// Create an empty [`Image`] with the given size. Uses [`Pixel::zero()`] to create all pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let image = Image::empty(Size::from_usize(2, 2).unwrap());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn empty(size: Size) -> Self {
        Self { size, pixels: vec![Pixel::zero(); size.area()].into_boxed_slice() }
    }

    /// Create a random [`Image`] with the given size. Uses [`Pixel::random()`] to create all
    /// pixels.
    ///
    /// Mainly useful for test scenarios.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let mut rng = rand::rng();
    /// let image = Image::random(Size::from_usize(2, 2).unwrap(), &mut rng);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn random<R>(size: Size, rng: &mut R) -> Self
    where
        R: Rng,
    {
        let pixels = from_fn(|| Some(Pixel::random(rng))).take(size.area()).collect();
        Self { size, pixels }
    }

    /// Get [`Image`]'s [`Size`].
    pub fn size(&self) -> Size {
        self.size
    }

    /// Get immutable [`Pixel`] at given `point`.
    ///
    /// Returns [`Pixel`] if point is within image bounds, [`OutOfBoundsError`] otherwise.
    ///
    /// [`OutOfBoundsError`]: crate::error::OutOfBoundsError
    pub fn pixel(&self, point: Point) -> IndexResult<&Pixel> {
        let index = point.index(self.size())?;

        // SAFETY: index from point.to_index is always valid
        Ok(&self.pixels[index])
    }

    /// Get mutable [`Pixel`] at given `point`.
    ///
    /// Returns [`Pixel`] if point is within image bounds, [`OutOfBoundsError`] otherwise.
    ///
    /// [`OutOfBoundsError`]: crate::error::OutOfBoundsError
    pub fn pixel_mut(&mut self, point: Point) -> IndexResult<&mut Pixel> {
        let index = point.index(self.size())?;

        // SAFETY: index from point.to_index is always valid
        Ok(&mut self.pixels[index])
    }

    /// Get raw `u8` buffer of underlying image data. It uses RGBA layout.
    pub fn buffer(&self) -> Box<[u8]> {
        self.pixels.iter().flat_map(|px| px.buffer()).cloned().collect()
    }

    /// Get [`ImageLens`] which borrows the [`Image`] to use with [`Lens`] API.
    pub fn lens(&self) -> ImageLens<'_> {
        ImageLens::new(self)
    }
}

impl<T: Into<Pixel>> FromLens<T> for Image {
    fn from_lens<S>(lens: S) -> Self
    where
        S: Lens<Item = T>,
    {
        let size = lens.size();
        let pixels = Box::from_iter(lens.elements().map(Into::into));

        // SAFETY: both size and pixels come from one Lens, which is guaranted
        // to return correct values.
        Self::new(size, pixels).unwrap()
    }
}

#[cfg(feature = "parallel")]
impl<T: Into<Pixel> + Send> FromLensPar<T> for Image {
    fn from_lens_par<S>(lens: S, threads: NonZeroUsize) -> Self
    where
        S: Lens<Item = T> + Send + Sync,
        S::Item: Send,
    {
        use std::thread;

        let size = lens.size();
        let threads = threads.get();
        let chunk_size = (size.area() as f32 / threads as f32).ceil() as usize;

        let mut image = Image::empty(size);

        let image_chunks = image.pixels.chunks_mut(chunk_size);

        thread::scope(|scope| {
            image_chunks.enumerate().for_each(|(index, chunk)| {
                let lens = &lens;
                scope.spawn(move || {
                    let starting_index = index * chunk_size;
                    chunk.iter_mut().enumerate().for_each(|(index, pixel)| {
                        // SAFETY: all starting_index + index will be in bounds since it enumerates
                        // over the image that it is indexing.
                        let point = Point::from_index(starting_index + index, size).unwrap();
                        // SAFETY: Lens::look is guaranted to return Ok if point is in bounds,
                        // and point is guaranted to be in bounds because of the check above.
                        *pixel = lens.look(point).unwrap().into();
                    });
                });
            });
        });

        image
    }
}
