#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

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
pub enum ResizeCreationError {
    #[error("new size is invalid: {0}")]
    NewSizeInvalid(#[from] SizeCreationError),
}

pub type ResizeCreationResult<T> = std::result::Result<T, ResizeCreationError>;

pub fn resize_lens<S>(source: S, scale: Scale) -> ResizeCreationResult<impl Lens<Item = Pixel>>
where
    S: Lens<Item = Pixel>,
{
    let size = scale.apply(source.size())?;
    let inverse_scale = scale.inverse();

    // SAFETY: if scale.apply was successful, then inverse_scale.translate will always be
    // successful
    let lens = source.remap(
        move |lens, point| {
            lens.look(inverse_scale.translate(point).expect("unexpected error in Scale::translate"))
        },
        size,
    );

    Ok(lens)
}

pub fn resize(image: &Image, scale: Scale) -> ResizeCreationResult<Image> {
    let lens = resize_lens(image.lens().cloned(), scale)?;
    Ok(Image::from_lens(lens))
}

#[cfg(feature = "parallel")]
pub fn resize_par(
    image: &Image,
    threads: NonZeroUsize,
    scale: Scale,
) -> ResizeCreationResult<Image> {
    use crate::lens::FromLensPar;

    let lens = resize_lens(image.lens().cloned(), scale)?;
    Ok(Image::from_lens_par(lens, threads))
}

#[cfg(test)]
mod tests {
    use rand::{
        SeedableRng,
        rngs::SmallRng,
    };

    use crate::prelude::Size;

    use super::*;

    #[test]
    fn test_resize_with_different_scales() {
        let image = Image::random(Size::new(10, 20).unwrap(), &mut SmallRng::seed_from_u64(0));

        let smaller = resize(&image, Scale::new(0.5, 0.5).unwrap());
        assert!(smaller.is_ok());
        assert_eq!(smaller.unwrap().size(), Size::new(5, 10).unwrap());

        let equal = resize(&image, Scale::new(1f32, 1f32).unwrap());
        assert!(equal.is_ok());
        assert_eq!(equal.unwrap().size(), Size::new(10, 20).unwrap());

        let larger = resize(&image, Scale::new(2f32, 2f32).unwrap());
        assert!(larger.is_ok());
        assert_eq!(larger.unwrap().size(), Size::new(20, 40).unwrap());

        let mixed = resize(&image, Scale::new(0.5, 2f32).unwrap());
        assert!(mixed.is_ok());
        assert_eq!(mixed.unwrap().size(), Size::new(5, 40).unwrap());
    }
}
