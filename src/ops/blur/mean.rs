use thiserror::Error;

use crate::image::Image;

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
pub fn mean_blur(image: &Image, radius: usize) -> Result<Image> {
    validate(image, radius)?;

    let diamater = radius * 2 + 1;
    let divisor_inv = 1f32 / (diamater * diamater) as f32;

    let mut new_image =
        Image::empty((image.size().0 - diamater + 1, image.size().1 - diamater + 1));

    new_image.rows_mut().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            let sum = (0..diamater)
                .flat_map(|k_y| {
                    (0..diamater).map(move |k_x| {
                        let new_px = unsafe { image.pixel_unchecked((x + k_x, y + k_y)) };
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
        });
    });

    Ok(new_image)
}

fn validate(image: &Image, radius: usize) -> Result<()> {
    if image.size().0 < radius * 2 + 1 || image.size().1 < radius * 2 + 1 {
        return Err(Error::RadiusTooBig);
    }

    Ok(())
}
