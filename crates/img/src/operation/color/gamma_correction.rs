use crate::{
    image::Image,
    lens::{
        FromLens,
        Lens,
    },
    pixel::{
        Pixel,
        PixelFlags,
        ReadPixelRgbaf32,
        WritePixelRgbaf32,
    },
};

pub fn gamma_correction_lens<S>(lens: S, gamma: f32, flags: PixelFlags) -> impl Lens<Item = Pixel>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    lens.map(move |px| map_px(px.as_ref(), gamma, flags))
}

pub fn gamma_correction(image: &Image, gamma: f32, flags: PixelFlags) -> Image {
    let lens = gamma_correction_lens(image.lens(), gamma, flags);
    Image::from_lens(lens)
}

#[cfg(feature = "parallel")]
pub fn gamma_correction_par(image: &Image, gamma: f32, flags: PixelFlags) -> Image {
    use crate::lens::FromLensPar;

    let lens = gamma_correction_lens(image.lens(), gamma, flags);
    Image::from_lens_par(lens)
}

fn map_px(px: &Pixel, gamma: f32, flags: PixelFlags) -> Pixel {
    let mut new_px = *px;

    new_px.set_with_flags_f32(
        px.r_f32().powf(gamma),
        px.g_f32().powf(gamma),
        px.b_f32().powf(gamma),
        px.a_f32().powf(gamma),
        flags,
    );

    new_px
}
