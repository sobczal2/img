use thiserror::Error;

use crate::image::Image;

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

pub fn crop(image: &Image, size: (usize, usize), offset: (usize, usize)) -> Result<Image> {
    validate(image, size, offset)?;

    let mut new_image = Image::empty(size);

    new_image.rows_mut().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            // SAFETY: validate ensures any x/ + offset is within bounds
            px.copy_from_pixel(unsafe { image.pixel_unchecked((x + offset.0, y + offset.1)) });
        });
    });

    Ok(new_image)
}

fn validate(image: &Image, size: (usize, usize), offset: (usize, usize)) -> Result<()> {
    if image.size().0 < size.0 || image.size().1 < size.1 {
        return Err(Error::SizeTooBig);
    }

    if image.size().0 < offset.0 || image.size().1 < offset.1 {
        return Err(Error::OffsetTooBig);
    }

    if image.size().0 < offset.0 + size.0 || image.size().1 < offset.1 + size.1 {
        return Err(Error::OutOfBounds);
    }

    Ok(())
}
