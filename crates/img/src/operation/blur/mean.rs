use thiserror::Error;

use crate::{component::kernel::mean::MeanKernel, image::Image, pipe::{FromPipe, Pipe}, primitive::size::Size};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum Error {
    #[error("radius too big for given image")]
    RadiusTooBig,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn mean_blur(image: &Image, radius: usize) -> Result<Image> {
    validate(image, radius)?;

    let kernel = MeanKernel::new(Size::from_radius(radius));
    let pipe = image.pipe().kernel(kernel).unwrap();

    Ok(Image::from_pipe(pipe))
}

fn validate(image: &Image, radius: usize) -> Result<()> {
    if image.size().width() < radius * 2 + 1 || image.size().height() < radius * 2 + 1 {
        return Err(Error::RadiusTooBig);
    }

    Ok(())
}
