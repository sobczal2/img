use crate::{
    image::Image,
    lens::{
        FromLens,
        FromLensPar,
        Lens,
    },
    pixel::Pixel,
    primitive::{
        margin::Margin,
        offset::Offset,
        size::{
            self,
        },
    },
};

pub fn crop_lens<S>(
    source: S,
    margin: Margin,
) -> Result<impl Lens<Item = Pixel>, size::CreationError>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    let size = source.size();
    let new_size = size.apply_margin(margin)?;

    Ok(source.remap(
        move |lens, point| {
            let top_left = margin.top_left();
            let original_point =
                point.offset_by(Offset::new(top_left.x() as isize, top_left.y() as isize)).unwrap();

            *lens.look(original_point).expect("bug in lens implementation").as_ref()
        },
        new_size,
    ))
}

pub fn crop(image: &Image, margin: Margin) -> Result<Image, size::CreationError> {
    let lens = crop_lens(image.lens(), margin)?;
    let image = Image::from_lens(lens);

    Ok(image)
}

#[cfg(feature = "parallel")]
pub fn crop_par(image: &Image, margin: Margin) -> Result<Image, size::CreationError> {
    let lens = crop_lens(image.lens(), margin)?;
    let image = Image::from_lens_par(lens);

    Ok(image)
}
