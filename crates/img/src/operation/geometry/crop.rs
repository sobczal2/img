#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use crate::{
    component::primitive::{
        Margin,
        Offset,
        SizeCreationError,
    },
    image::Image,
    lens::{
        FromLens,
        FromLensPar,
        Lens,
    },
    pixel::Pixel,
};

pub fn crop_lens<S>(source: S, margin: Margin) -> Result<impl Lens<Item = Pixel>, SizeCreationError>
where
    S: Lens<Item = Pixel>,
{
    let size = source.size();
    let new_size = size.apply_margin(margin)?;

    Ok(source.remap(
        move |lens, point| {
            // SAFETY: translate returns Err only if resulting offset is negative,
            // here offset we apply has always positive values, so the resulting
            // offset will also always be positive.
            let original_point = point
                .translate(Offset::new(margin.left() as isize, margin.top() as isize))
                .unwrap();

            lens.look(original_point)
        },
        new_size,
    ))
}

pub fn crop(image: &Image, margin: Margin) -> Result<Image, SizeCreationError> {
    let lens = crop_lens(image.lens().cloned(), margin)?;
    let image = Image::from_lens(lens);

    Ok(image)
}

#[cfg(feature = "parallel")]
pub fn crop_par(
    image: &Image,
    threads: NonZeroUsize,
    margin: Margin,
) -> Result<Image, SizeCreationError> {
    let lens = crop_lens(image.lens().cloned(), margin)?;
    let image = Image::from_lens_par(lens, threads);

    Ok(image)
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::prelude::Size;

    use super::*;

    #[test]
    fn test_crop_with_valid_margins() {
        let image = Image::random(Size::from_usize(10, 20).unwrap(), &mut StdRng::from_seed([7u8; 32]));

        let equal = crop(&image, Margin::new(0, 0, 0, 0));
        assert!(equal.is_ok());
        assert_eq!(equal.unwrap().size(), Size::from_usize(10, 20).unwrap());

        let top_right = crop(&image, Margin::new(5, 5, 0, 0));
        assert!(top_right.is_ok());
        assert_eq!(top_right.unwrap().size(), Size::from_usize(5, 15).unwrap());

        let bottom_left = crop(&image, Margin::new(0, 0, 5, 5));
        assert!(bottom_left.is_ok());
        assert_eq!(bottom_left.unwrap().size(), Size::from_usize(5, 15).unwrap());
    }

    #[test]
    fn test_crop_with_invalid_margins() {
        let image = Image::random(Size::from_usize(10, 20).unwrap(), &mut StdRng::from_seed([7u8; 32]));

        let shrinked_horizontal = crop(&image, Margin::new(0, 5, 0, 5));
        assert_eq!(shrinked_horizontal.unwrap_err(), SizeCreationError::WidthZero);

        let shrinked_vertical = crop(&image, Margin::new(10, 0, 10, 0));
        assert_eq!(shrinked_vertical.unwrap_err(), SizeCreationError::HeightZero);

        let shrinked_both = crop(&image, Margin::new(10, 5, 10, 5));
        assert_eq!(shrinked_both.unwrap_err(), SizeCreationError::WidthZero);

        let top_oob = crop(&image, Margin::new(20, 0, 0, 0));
        assert_eq!(top_oob.unwrap_err(), SizeCreationError::HeightZero);

        let right_oob = crop(&image, Margin::new(0, 10, 0, 0));
        assert_eq!(right_oob.unwrap_err(), SizeCreationError::WidthZero);

        let bottom_oob = crop(&image, Margin::new(0, 0, 20, 0));
        assert_eq!(bottom_oob.unwrap_err(), SizeCreationError::HeightZero);

        let left_oob = crop(&image, Margin::new(0, 0, 0, 10));
        assert_eq!(left_oob.unwrap_err(), SizeCreationError::WidthZero);
    }
}
