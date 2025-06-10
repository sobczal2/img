#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::image::Image;

fn calculate_size(size: (usize, usize), scale: (f32, f32)) -> (usize, usize) {
    let new_size = (size.0 as f32 * scale.0.abs(), size.1 as f32 * scale.1.abs());
    (new_size.0 as usize, new_size.1 as usize)
}

fn nearest(xy: (usize, usize), scale: (f32, f32)) -> (usize, usize) {
    debug_assert_ne!(scale.0, 0f32);
    debug_assert_ne!(scale.1, 0f32);

    (
        (xy.0 as f32 / scale.0) as usize,
        (xy.1 as f32 / scale.1) as usize,
    )
}

pub fn resize(image: &Image, scale: (f32, f32)) -> Image {
    if scale.0 == 0f32 || scale.1 == 0f32 {
        return Image::empty((0, 0));
    }

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

    new_image
}
