#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};
use thiserror::Error;

use crate::image::Image;

#[derive(Debug, Error)]
pub enum Error {
    #[error("size is zero")]
    SizeZero,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn resize(image: &Image, scale: (f32, f32)) -> Result<Image> {
    validate(image, scale)?;
    let new_size = calculate_size(image.size(), scale);
    let mut new_image = Image::empty(new_size);

    #[cfg(feature = "parallel")]
    new_image.rows_mut().par_bridge().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            // SAFETY: nearest function should always return a valid point in original image
            px.copy_from_pixel(unsafe { image.pixel_unchecked(nearest((x, y), scale)) });
        });
    });

    #[cfg(not(feature = "parallel"))]
    new_image.rows_mut().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            // SAFETY: nearest function should always return a valid point in original image
            px.copy_from_pixel(unsafe { image.pixel_unchecked(nearest((x, y), scale)) });
        });
    });

    Ok(new_image)
}

fn validate(image: &Image, scale: (f32, f32)) -> Result<()> {
    let min_scale = (1f32 / image.size().0 as f32, 1f32 / image.size().0 as f32);
    if scale.0.abs() < min_scale.0 || scale.1.abs() < min_scale.1 {
        return Err(Error::SizeZero);
    }

    Ok(())
}

fn calculate_size(size: (usize, usize), scale: (f32, f32)) -> (usize, usize) {
    let new_size = (size.0 as f32 * scale.0.abs(), size.1 as f32 * scale.1.abs());
    (new_size.0 as usize, new_size.1 as usize)
}

fn nearest(xy: (usize, usize), scale: (f32, f32)) -> (usize, usize) {
    (
        (xy.0 as f32 / scale.0) as usize,
        (xy.1 as f32 / scale.1) as usize,
    )
}
