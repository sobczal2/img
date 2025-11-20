use crate::{
    component::primitive::{
        Margin,
        Point,
    },
    error::IndexResult,
    lens::Lens,
};

pub mod convolution;
pub mod gaussian;
pub mod identity;
pub mod mean;
pub mod sobel;

/// A trait for describing how to evaluate value for a [`Point`] based on
/// elements within certain margin.
pub trait Kernel<In, Out> {

    /// Evaluate value for a point based on elements within certain marin.
    fn evaluate<S>(&self, source: &S, point: Point) -> IndexResult<Out>
    where
        S: Lens<Item = In>;

    /// Get margin that is used for computation.
    fn margin(&self) -> Margin;
}
