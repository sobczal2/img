use crate::{
    component::{
        kernel::Kernel,
        primitive::{
            Point,
            Size,
        },
    },
    error::IndexResult,
    lens::{
        cloned::ClonedLens,
        iter::{
            Elements,
            Rows,
        },
        kernel::KernelLens,
        map::MapLens,
        materialize::MaterializeLens,
        overlay::OverlayLens,
        remap::RemapLens,
        split::{
            SplitLens2,
            SplitLens3,
            SplitLens4,
        },
    },
};

pub mod cloned;
pub mod image;
pub mod iter;
pub mod kernel;
pub mod map;
pub mod materialize;
pub mod overlay;
pub mod remap;
pub mod split;
pub mod value;

/// A trait for chaining operations for a 2D structures.
///
/// This is main way for applying transformations and change `Image`.
pub trait Lens {
    /// Type of individual items within underlying 2d structure. This can be
    /// `Pixel`, but this is not a requirement.
    type Item;

    /// Look at value for given `point`.
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
    /// use img::prelude::*;
    /// use img::lens::Lens;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let image = Image::empty(Size::from_usize(10, 20)?);
    ///
    /// let lens = image.lens();
    ///
    /// assert!(lens.look(Point::new(0, 0)).is_ok());
    /// assert!(lens.look(Point::new(9, 0)).is_ok());
    /// assert!(lens.look(Point::new(0, 19)).is_ok());
    /// assert!(lens.look(Point::new(10, 0)).is_err());
    /// assert!(lens.look(Point::new(0, 20)).is_err());
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn look(&self, point: Point) -> IndexResult<Self::Item>;

    /// Get size of lens's output. This should be aligned with the behaviour of
    /// `get()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// use img::lens::Lens;
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
    /// assert!(lens.look(valid_point).is_ok());
    /// assert!(lens.size().contains(&valid_point));
    /// assert!(lens.look(invalid_point).is_err());
    /// assert!(!lens.size().contains(&invalid_point));
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
        F: Fn(&Self, Point) -> IndexResult<T>,
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

    fn materialize(self) -> MaterializeLens<Self::Item>
    where
        Self: Sized,
    {
        MaterializeLens::new(self)
    }

    #[cfg(feature = "parallel")]
    fn materialize_par(self) -> MaterializeLens<Self::Item>
    where
        Self: Sized + Send + Sync,
        Self::Item: Send,
    {
        MaterializeLens::new_par(self)
    }

    fn split2<F1, F2, L1, L2, D1, D2>(self, factory1: F1, factory2: F2) -> SplitLens2<L1, L2>
    where
        Self: Sized + Clone,
        F1: Fn(Self) -> L1,
        F2: Fn(Self) -> L2,
        L1: Lens<Item = D1>,
        L2: Lens<Item = D2>,
    {
        SplitLens2::new(self, factory1, factory2)
    }

    fn split3<F1, F2, F3, L1, L2, L3, D1, D2, D3>(
        self,
        factory1: F1,
        factory2: F2,
        factory3: F3,
    ) -> SplitLens3<L1, L2, L3>
    where
        Self: Sized + Clone,
        F1: Fn(Self) -> L1,
        F2: Fn(Self) -> L2,
        F3: Fn(Self) -> L3,
        L1: Lens<Item = D1>,
        L2: Lens<Item = D2>,
        L3: Lens<Item = D3>,
    {
        SplitLens3::new(self, factory1, factory2, factory3)
    }

    fn split4<F1, F2, F3, F4, L1, L2, L3, L4, D1, D2, D3, D4>(
        self,
        factory1: F1,
        factory2: F2,
        factory3: F3,
        factory4: F4,
    ) -> SplitLens4<L1, L2, L3, L4>
    where
        Self: Sized + Clone,
        F1: Fn(Self) -> L1,
        F2: Fn(Self) -> L2,
        F3: Fn(Self) -> L3,
        F4: Fn(Self) -> L4,
        L1: Lens<Item = D1>,
        L2: Lens<Item = D2>,
        L3: Lens<Item = D3>,
        L4: Lens<Item = D4>,
    {
        SplitLens4::new(self, factory1, factory2, factory3, factory4)
    }

    fn overlay<S>(
        self,
        overlay: S,
        overlay_start: Point,
    ) -> overlay::CreationResult<OverlayLens<Self, S>>
    where
        Self: Sized,
        S: Lens<Item = Self::Item>,
    {
        OverlayLens::new(self, overlay, overlay_start)
    }
}

pub trait FromLens<T>: Sized {
    fn from_lens<S>(source: S) -> Self
    where
        S: Lens<Item = T>;
}

#[cfg(feature = "parallel")]
pub trait FromLensPar<T>: Sized {
    fn from_lens_par<S>(source: S) -> Self
    where
        S: Lens<Item = T> + Send + Sync,
        S::Item: Send;
}
