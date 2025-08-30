#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use thiserror::Error;

use crate::{
    image::Image,
    primitive::{point::Point, size::Size},
};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum Error {
    #[error("radius too big for given image")]
    RadiusTooBig,
}

pub type Result<T> = std::result::Result<T, Error>;

/// perform mean blur on an image not in place
/// this reduces size of an image by radius * 2 times
/// so to receive image of an original size you should pad it
#[cfg(false)]
pub fn mean_blur(image: &Image, radius: usize) -> Result<Image> {
    validate(image, radius)?;

    let diamater = radius * 2 + 1;

    let mut new_image = Image::empty(
        Size::from_usize(
            image.size().width() - diamater + 1,
            image.size().height() - diamater + 1,
        )
        .unwrap(),
    );

    new_image.rows_mut().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            process_pixel(Point::new(x, y), &mut px, image, radius);
        });
    });

    Ok(new_image)
}

#[cfg(feature = "parallel")]
pub fn mean_blur_par(image: &Image, radius: usize) -> Result<Image> {
    validate(image, radius)?;

    let diamater = radius * 2 + 1;

    let mut new_image = Image::empty(
        Size::from_usize(
            image.size().width() - diamater + 1,
            image.size().height() - diamater + 1,
        )
        .unwrap(),
    );

    new_image.rows_mut().par_bridge().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            process_pixel(Point::new(x, y), &mut px, image, radius);
        });
    });

    Ok(new_image)
}

fn validate(image: &Image, radius: usize) -> Result<()> {
    if image.size().width() < radius * 2 + 1 || image.size().height() < radius * 2 + 1 {
        return Err(Error::RadiusTooBig);
    }

    Ok(())
}

#[cfg(false)]
fn process_pixel(point: Point, px: &mut PixelMut, original_image: &Image, radius: usize) {
    let diameter = radius * 2 + 1;
    let divisor_inv = 1f32 / (diameter * diameter) as f32;

    let sum = (0..diameter)
        .flat_map(|k_y| {
            (0..diameter).map(move |k_x| {
                let new_px = unsafe {
                    original_image.pixel_unchecked(Point::new(point.x() + k_x, point.y() + k_y))
                };
                (new_px.r(), new_px.g(), new_px.b())
            })
        })
        .fold((0, 0, 0), |(acc_r, acc_g, acc_b), (r, g, b)| {
            (acc_r + r as u32, acc_g + g as u32, acc_b + b as u32)
        });

    px.set_r((sum.0 as f32 * divisor_inv) as u8);
    px.set_g((sum.1 as f32 * divisor_inv) as u8);
    px.set_b((sum.2 as f32 * divisor_inv) as u8);
    px.set_a(255);
}
