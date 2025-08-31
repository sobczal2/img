use crate::{
    component::kernel::Kernel,
    error::IndexResult,
    pipe::{
        cloned::ClonedPipe,
        iter::{Elements, Rows},
        kernel::{KernelPipe, KernelPipeCreationError},
        map::MapPipe,
        remap::RemapPipe,
    },
    primitive::{point::Point, size::Size},
};

pub mod cloned;
pub mod image;
pub mod iter;
pub mod kernel;
pub mod map;
pub mod remap;

/// A trait for chaining operations for a 2D structures.
///
/// This is main way for applying transformations and change `Image`.
pub trait Pipe {
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
    /// use img::primitive::{point::Point, size::Size};
    /// use img::image::Image;
    /// use img::pipe::Pipe;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let image = Image::empty(Size::from_usize(10, 20)?);
    ///
    /// let pipe = image.pipe();
    ///
    /// assert!(pipe.get(Point::new(0, 0)).is_ok());
    /// assert!(pipe.get(Point::new(9, 0)).is_ok());
    /// assert!(pipe.get(Point::new(0, 19)).is_ok());
    /// assert!(pipe.get(Point::new(10, 0)).is_err());
    /// assert!(pipe.get(Point::new(0, 20)).is_err());
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn get(&self, point: Point) -> IndexResult<Self::Item>;

    /// Get size of pipe's output. This should be aligned with the behaviour of
    /// `get()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::primitive::{point::Point, size::Size};
    /// use img::image::Image;
    /// use img::pipe::Pipe;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let size = Size::from_usize(10,20)?;
    /// let image = Image::empty(size);
    ///
    /// let pipe = image.pipe();
    ///
    /// assert_eq!(pipe.size(), size);
    ///
    /// let valid_point = Point::new(0, 0);
    /// let invalid_point = Point::new(10, 0);
    ///
    /// assert!(pipe.get(valid_point).is_ok());
    /// assert!(pipe.size().contains(valid_point));
    /// assert!(pipe.get(invalid_point).is_err());
    /// assert!(!pipe.size().contains(invalid_point));
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

    fn map<T, F>(self, f: F) -> MapPipe<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Item) -> T,
    {
        MapPipe::new(self, f)
    }

    fn remap<T, F>(self, f: F, size: Size) -> RemapPipe<Self, F>
    where
        Self: Sized,
        F: Fn(&Self, Point) -> T,
    {
        RemapPipe::new(self, f, size)
    }

    fn cloned<'a>(self) -> ClonedPipe<Self>
    where
        Self: Sized,
        Self::Item: Clone + 'a,
    {
        ClonedPipe::new(self)
    }

    fn kernel<K, T>(self, kernel: K) -> Result<KernelPipe<Self, K, T>, KernelPipeCreationError>
    where
        Self: Sized,
        K: Kernel<Self::Item, T>,
    {
        KernelPipe::new(self, kernel)
    }
}

pub trait FromPipe<T>: Sized {
    fn from_pipe<P>(pipe: P) -> Self
    where
        P: Pipe<Item = T>;
}

#[cfg(feature = "parallel")]
pub trait FromPipePar<T>: Sized {
    fn from_pipe_par<P>(pipe: P) -> Self
    where
        P: Pipe<Item = T> + Send + Sync,
        P::Item: Send;
}
