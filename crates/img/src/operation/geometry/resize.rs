use thiserror::Error;

use crate::{
    image::Image,
    pipe::{FromPipe, Pipe},
    primitive::{scale::Scale, size},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("new size is invalid: {0}")]
    NewSizeInvalid(#[from] size::CreationError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn resize(image: &Image, scale: Scale) -> Result<Image> {
    let size = scale.apply(image.size())?;
    let inverse_scale = scale.inverse();

    let pipe = image
        .pipe()
        .remap(
            |pipe, point| {
                pipe.get(inverse_scale.translate(point))
                    .expect("out of bounds in resize")
            },
            size,
        )
        .cloned();

    let image = Image::from_pipe(pipe);

    Ok(image)
}
