use thiserror::Error;

use crate::{
    component::primitive::{
        Scale,
        SizeCreationError,
    },
    image::Image,
    lens::{
        FromLens,
        Lens,
    },
    pixel::Pixel,
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("new size is invalid: {0}")]
    NewSizeInvalid(#[from] SizeCreationError),
}

pub fn resize_lens<S>(source: S, scale: Scale) -> Result<impl Lens<Item = Pixel>, CreationError>
where
    S: Lens<Item = Pixel>,
{
    let size = scale.apply(source.size())?;
    let inverse_scale = scale.inverse();

    let lens = source.remap(move |lens, point| lens.look(inverse_scale.translate(point)), size);

    Ok(lens)
}

pub fn resize(image: &Image, scale: Scale) -> Result<Image, CreationError> {
    let lens = resize_lens(image.lens().cloned(), scale)?;
    Ok(Image::from_lens(lens))
}

#[cfg(feature = "parallel")]
pub fn resize_par(image: &Image, scale: Scale) -> Result<Image, CreationError> {
    use crate::lens::FromLensPar;

    let lens = resize_lens(image.lens().cloned(), scale)?;
    Ok(Image::from_lens_par(lens))
}
