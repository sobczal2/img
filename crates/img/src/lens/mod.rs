use crate::{
    component::kernel::Kernel,
    error::IndexResult,
    lens::{
        cloned::ClonedLens,
        iter::{
            Elements,
            Rows,
        },
        kernel::KernelLens,
        map::MapLens,
        remap::RemapLens,
    },
    primitive::{
        point::Point,
        size::Size,
    },
};

pub mod cloned;
pub mod image;
pub mod iter;
pub mod kernel;
pub mod map;
pub mod materialize;
pub mod remap;

/// A trait for chaining operations for a 2D structures.
///
/// This is main way for applying transformations and change `Image`.
pub trait Lens {
    /// Type of individual items within underlying 2d structure. This can be
    /// `Pixel`, but this is not a requirement.
    type Item;

    /// Read value at `point`.
    ///
    /// Returns `Ok(Self::Item)` if point is within bounds, `OutOfBoundsError`
    /// otherwise. This should always return a value when `point` is contained
    /// in `size()`, error otherwise. Each implementation should behave like
    /// this, it leads to bugs otherwise.
    ///
    /// Most implementations will not perform any costly calculations until
    /// this method is called. Also this method should invoke only calculations
    /// directly related to requested `Point`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     image::Image,
    ///     lens::Lens,
    ///     primitive::{
    ///         point::Point,
    ///         size::Size,
    ///     },
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let image = Image::empty(Size::from_usize(10, 20)?);
    ///
    /// let lens = image.lens();
    ///
    /// assert!(lens.get(Point::new(0, 0)).is_ok());
    /// assert!(lens.get(Point::new(9, 0)).is_ok());
    /// assert!(lens.get(Point::new(0, 19)).is_ok());
    /// assert!(lens.get(Point::new(10, 0)).is_err());
    /// assert!(lens.get(Point::new(0, 20)).is_err());
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn get(&self, point: Point) -> IndexResult<Self::Item>;

    /// Get size of lens's output. This should be aligned with the behaviour of
    /// `get()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::{
    ///     image::Image,
    ///     lens::Lens,
    ///     primitive::{
    ///         point::Point,
    ///         size::Size,
    ///     },
    /// };
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::from_usize(10, 20)?;
    /// let image = Image::empty(size);
    ///
    /// let lens = image.lens();
    ///
    /// assert_eq!(lens.size(), size);
    ///
    /// let valid_point = Point::new(0, 0);
    /// let invalid_point = Point::new(10, 0);
    ///
    /// assert!(lens.get(valid_point).is_ok());
    /// assert!(lens.size().contains(valid_point));
    /// assert!(lens.get(invalid_point).is_err());
    /// assert!(!lens.size().contains(invalid_point));
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn size(&self) -> Size;

    fn rows(&self) -> Rows<'_, Self>
    where
        Self: Sized,
    {
        Rows::new(self)
    }

    fn elements(&self) -> Elements<'_, Self>
    where
        Self: Sized,
    {
        Elements::new(self)
    }

    fn map<T, F>(self, f: F) -> MapLens<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Item) -> T,
    {
        MapLens::new(self, f)
    }

    fn remap<T, F>(self, f: F, size: Size) -> RemapLens<Self, F>
    where
        Self: Sized,
        F: Fn(&Self, Point) -> T,
    {
        RemapLens::new(self, f, size)
    }

    fn cloned<'a>(self) -> ClonedLens<Self>
    where
        Self: Sized,
        Self::Item: Clone + 'a,
    {
        ClonedLens::new(self)
    }

    fn kernel<K, T>(self, kernel: K) -> Result<KernelLens<Self, K, T>, kernel::CreationError>
    where
        Self: Sized,
        K: Kernel<Self::Item, T>,
    {
        KernelLens::new(self, kernel)
    }
}

pub trait FromLens<T>: Sized {
    fn from_lens<P>(lens: P) -> Self
    where
        P: Lens<Item = T>;
}

#[cfg(feature = "parallel")]
pub trait FromLensPar<T>: Sized {
    fn from_lens_par<P>(lens: P) -> Self
    where
        P: Lens<Item = T> + Send + Sync,
        P::Item: Send;
}
