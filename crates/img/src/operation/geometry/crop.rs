use std::cmp::Ordering;

use thiserror::Error;

use crate::{
    image::Image,
    pipe::{FromPipe, Pipe},
    primitive::{point::Point, size::Size},
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

    let pipe = image
        .pipe()
        .remap(
            |pipe, point| {
                pipe.get(Point::new(point.x() + offset.0, point.y() + offset.1))
                    .unwrap()
            },
            size,
        )
        .cloned();
    let image = Image::from_pipe(pipe);

    Ok(image)
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
