use thiserror::Error;

use crate::{
    component::kernel::gaussian::GaussianKernel,
    image::Image,
    pipe::{FromPipe, Pipe},
    primitive::size::Size,
};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum Error {
    #[error("radius too big for given image")]
    RadiusTooBig,
    #[error("invalid sigma - has to be positive")]
    InvalidSigma,
}

pub type Result<T> = std::result::Result<T, Error>;

/// perform mean blur on an image not in place
/// this reduces size of an image by radius * 2 times
/// so to receive image of an original size you should pad it
pub fn gaussian_blur(image: &Image, radius: usize, sigma: f32) -> Result<Image> {
    validate(image, radius, sigma)?;

    let kernel = GaussianKernel::new(Size::from_radius(radius), sigma);
    let pipe = image.pipe().kernel(kernel).unwrap();

    Ok(Image::from_pipe(pipe))
}

fn validate(image: &Image, radius: usize, sigma: f32) -> Result<()> {
    if image.size().width() < radius * 2 + 1 || image.size().height() < radius * 2 + 1 {
        return Err(Error::RadiusTooBig);
    }

    if sigma <= 0f32 {
        return Err(Error::InvalidSigma);
    }

    Ok(())
}
