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
pub enum CreationError {
    #[error("size does not match pixels size")]
    SizePixelsMismatch,
}

pub type ResultError<T> = Result<T, CreationError>;

/// Maximum dimension size of an image (width or height).
/// Guaranted to be less than isize::MAX.
#[cfg(target_pointer_width = "64")]
pub const DIMENSION_MAX: usize = (1u64 << 32) as usize - 1;

#[cfg(target_pointer_width = "32")]
pub const DIMENSION_MAX: usize = (1u32 << 16) as usize - 1;

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
    ///     image::CreationError,
    ///     prelude::*,
    /// };
    /// use std::iter::from_fn;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::new(2, 2)?;
    /// let bad_size = Size::new(2, 3)?;
    /// let pixels = vec![Pixel::zero(); size.area()].into_boxed_slice();
    /// let image = Image::new(size, pixels.clone())?;
    ///
    /// let mismatch = Image::new(bad_size, pixels);
    /// assert_eq!(mismatch.unwrap_err(), CreationError::SizePixelsMismatch);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(size: Size, pixels: Box<[Pixel]>) -> ResultError<Self> {
        if pixels.len() != size.area() {
            return Err(CreationError::SizePixelsMismatch);
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
    /// let image = Image::empty(Size::new(2, 2)?);
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
    /// let image = Image::random(Size::new(2, 2)?, &mut rng);
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
    /// Collect [`Lens`] into an [`Image`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     lens::{
    ///         FromLens,
    ///         value::ValueLens,
    ///     },
    ///     prelude::*,
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let lens = ValueLens::new(Pixel::zero(), Size::new(2, 2)?);
    /// let image = Image::from_lens(lens);
    ///
    /// assert_eq!(image.size(), Size::new(2, 2)?);
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn from_lens<S>(lens: S) -> Self
    where
        S: Lens<Item = T>,
    {
        let size = lens.size();
        let pixels = Box::from_iter(lens.elements().map(Into::into));

        // SAFETY: both size and pixels come from one Lens, which is guaranted
        // to return correct values.
        Self::new(size, pixels).expect("bug in lens implementation")
    }
}

#[cfg(feature = "parallel")]
impl<T: Into<Pixel> + Send> FromLensPar<T> for Image {
    /// Collect [`Lens`] into an [`Image`].
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     lens::{
    ///         FromLensPar,
    ///         value::ValueLens,
    ///     },
    ///     prelude::*,
    /// };
    /// use std::num::NonZero;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let lens = ValueLens::new(Pixel::zero(), Size::new(2, 2).unwrap());
    /// let image = Image::from_lens_par(lens, NonZero::new(3).unwrap());
    ///
    /// assert_eq!(image.size(), Size::new(2, 2).unwrap());
    ///
    /// # Ok(())
    /// # }
    /// ```
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
                        let point = Point::from_index(starting_index + index, size).expect("unexpected error calculating index");
                        // SAFETY: Lens::look is guaranted to return Ok if point is in bounds,
                        // and point is guaranted to be in bounds because of the check above.
                        *pixel = lens.look(point).expect("unexpected error in Lens::look").into();
                    });
                });
            });
        });

        image
    }
}

#[cfg(test)]
mod tests {
    use std::{isize, u128};

    use itertools::Itertools;
    use rand::{
        SeedableRng,
        rngs::SmallRng,
    };

    use crate::error::IndexError;

    use super::*;

    #[test]
    fn test_dimension_max() {
        assert!(DIMENSION_MAX < isize::MAX as usize);
        assert!((DIMENSION_MAX as u128 * DIMENSION_MAX as u128) < usize::MAX as u128);
    }

    #[test]
    fn test_new_err() {
        let size = Size::new(2, 2).unwrap();
        let bad_size = Size::new(2, 3).unwrap();
        let pixels = vec![Pixel::zero(); size.area()].into_boxed_slice();
        let image = Image::new(bad_size, pixels.clone());
        assert_eq!(image.unwrap_err(), CreationError::SizePixelsMismatch);
    }

    #[test]
    fn test_empty() {
        let size = Size::new(2, 2).unwrap();
        let image = Image::empty(size);
        assert_eq!(size, image.size());
        for p in image.pixels.iter() {
            assert_eq!(p, &Pixel::zero());
        }
    }

    #[test]
    fn test_random() {
        let size = Size::new(2, 2).unwrap();
        let image = Image::random(size, &mut SmallRng::seed_from_u64(0));
        assert_eq!(size, image.size());
        assert_eq!(image.pixels.len(), size.area());
    }

    #[test]
    fn test_pixel_ok() {
        let size = Size::new(2, 2).unwrap();
        let image = Image::empty(size);
        let point = Point::new(1, 1);
        assert_eq!(image.pixel(point).unwrap(), &Pixel::zero());
    }

    #[test]
    fn test_pixel_oob() {
        let size = Size::new(2, 2).unwrap();
        let image = Image::empty(size);
        let point = Point::new(2, 1);
        assert_eq!(image.pixel(point).unwrap_err(), IndexError::OutOfBounds);
    }

    #[test]
    fn test_pixel_mut_ok() {
        let size = Size::new(2, 2).unwrap();
        let mut image = Image::empty(size);
        let point = Point::new(1, 1);
        assert_eq!(image.pixel_mut(point).unwrap(), &Pixel::zero());
    }

    #[test]
    fn test_pixel_mut_oob() {
        let size = Size::new(2, 2).unwrap();
        let mut image = Image::empty(size);
        let point = Point::new(2, 1);
        assert_eq!(image.pixel_mut(point).unwrap_err(), IndexError::OutOfBounds);
    }

    #[test]
    fn test_buffer_consistency() {
        let size = Size::new(2, 1).unwrap();
        let pixels = vec![Pixel::new([1, 2, 3, 4]), Pixel::new([5, 6, 7, 8])].into_boxed_slice();

        let image = Image::new(size, pixels).unwrap();
        let buffer = image.buffer();

        assert_eq!(buffer, vec![1, 2, 3, 4, 5, 6, 7, 8].into_boxed_slice());
    }

    #[test]
    fn test_lens() {
        let size = Size::new(2, 2).unwrap();
        let image = Image::random(size, &mut SmallRng::seed_from_u64(0));
        let lens = image.lens();

        assert_eq!(image.size(), lens.size());
        for (x, y) in (0..2).cartesian_product(0..2) {
            let point = Point::new(x, y);
            assert_eq!(image.pixel(point).unwrap(), lens.look(point).unwrap());
        }
    }

    #[test]
    fn test_from_lens() {
        let size = Size::new(2, 2).unwrap();
        let image1 = Image::random(size, &mut SmallRng::seed_from_u64(0));
        let image2 = Image::from_lens(image1.lens().cloned());

        assert_eq!(image1.size(), image2.size());
        for (x, y) in (0..2).cartesian_product(0..2) {
            let point = Point::new(x, y);
            assert_eq!(image1.pixel(point).unwrap(), image2.pixel(point).unwrap());
        }
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_from_lens_par() {
        use std::num::NonZero;
        let size = Size::new(2, 2).unwrap();
        let image1 = Image::random(size, &mut SmallRng::seed_from_u64(0));
        let image2 = Image::from_lens_par(image1.lens().cloned(), NonZero::new(4).unwrap());

        assert_eq!(image1.size(), image2.size());
        for (x, y) in (0..2).cartesian_product(0..2) {
            let point = Point::new(x, y);
            assert_eq!(image1.pixel(point).unwrap(), image2.pixel(point).unwrap());
        }
    }
}
