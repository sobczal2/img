use std::cmp::Ordering;

use thiserror::Error;

use crate::{
    image::Image,
    primitives::{point::Point, size::Size},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("offset is bigger than image size")]
    OffsetTooBig,
    #[error("target size too big")]
    SizeTooBig,
    #[error("crop area is out of bounds")]
    OutOfBounds,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn crop(image: &Image, size: Size, offset: (usize, usize)) -> Result<Image> {
    validate(image, size, offset)?;

    let mut new_image = Image::empty(size);

    new_image.rows_mut().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            // SAFETY: validate ensures any x/ + offset is within bounds
            px.copy_from_pixel(unsafe {
                image.pixel_unchecked(Point::new(x + offset.0, y + offset.1))
            });
        });
    });

    Ok(new_image)
}

fn validate(image: &Image, size: Size, offset: (usize, usize)) -> Result<()> {
    if image.size().partial_cmp(&size) != Some(Ordering::Greater) {
        return Err(Error::SizeTooBig);
    }

    if image.size().width() < offset.0 || image.size().height() < offset.1 {
        return Err(Error::OffsetTooBig);
    }

    if image.size().width() < offset.0 + size.width()
        || image.size().height() < offset.1 + size.height()
    {
        return Err(Error::OutOfBounds);
    }

    Ok(())
}
