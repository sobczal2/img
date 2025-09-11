use crate::{
    image::Image,
    lens::{
        FromLens,
        Lens,
    },
    pixel::{
        ChannelFlags,
        Pixel,
    },
};

pub fn grayscale_lens<S>(source: S, flags: ChannelFlags) -> impl Lens<Item = Pixel>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    source.map(move |px| map_px(px, flags))
}

fn map_px(px: impl AsRef<Pixel>, flags: ChannelFlags) -> Pixel {
    let px = px.as_ref();
    let value = 0.299 * px.r() as f32 + 0.587 * px.g() as f32 + 0.214 * px.b() as f32;
    let value = value as u8;

    let mut px = *px;
    px.set_with_flags(value, value, value, value, flags);

    px
}

pub fn grayscale(image: &Image, flags: ChannelFlags) -> Image {
    let lens = grayscale_lens(image.lens(), flags);
    Image::from_lens(lens)
}

#[cfg(feature = "parallel")]
pub fn grayscale_par(image: &Image, flags: ChannelFlags) -> Image {
    use crate::lens::FromLensPar;

    let lens = grayscale_lens(image.lens(), flags);
    Image::from_lens_par(lens)
}
