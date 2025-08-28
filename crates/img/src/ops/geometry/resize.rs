#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use thiserror::Error;

use crate::{
    image::Image,
    primitives::{point::Point, scale::Scale, size::SizeCreationError},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("new size is invalid: {0}")]
    NewSizeInvalid(#[from] SizeCreationError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn resize(image: &Image, scale: Scale) -> Result<Image> {
    let new_size = scale.apply(image.size())?;
    let mut new_image = Image::empty(new_size);

    let inverse_scale = scale.inverse();

    new_image.rows_mut().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            // SAFETY: nearest function should always return a valid point in original image
            px.copy_from_pixel(unsafe { image.pixel_unchecked(inverse_scale.translate(Point::new(x, y))) });
        });
    });

    Ok(new_image)
}

#[cfg(feature = "parallel")]
pub fn resize_par(image: &Image, scale: Scale) -> Result<Image> {
    let new_size = scale.apply(image.size())?;
    let mut new_image = Image::empty(new_size);

    let inverse_scale = scale.inverse();

    new_image.rows_mut().par_bridge().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            // SAFETY: nearest function should always return a valid point in original image
            px.copy_from_pixel(unsafe { image.pixel_unchecked(inverse_scale.translate(Point::new(x, y))) });
        });
    });

    Ok(new_image)
}
