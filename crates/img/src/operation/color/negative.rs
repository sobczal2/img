#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

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

pub fn negative_lens<S>(source: S, flags: ChannelFlags) -> impl Lens<Item = Pixel>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    source.map(move |px| map_px(px, flags))
}

pub fn negative(image: &Image, flags: ChannelFlags) -> Image {
    let lens = negative_lens(image.lens(), flags);
    Image::from_lens(lens)
}

#[cfg(feature = "parallel")]
pub fn negative_par(image: &Image, threads: NonZeroUsize, flags: ChannelFlags) -> Image {
    use crate::lens::FromLensPar;

    let lens = negative_lens(image.lens(), flags);
    Image::from_lens_par(lens, threads)
}

fn map_px(px: impl AsRef<Pixel>, flags: ChannelFlags) -> Pixel {
    let px = px.as_ref();

    let new_red = u8::MAX - px.r();
    let new_green = u8::MAX - px.g();
    let new_blue = u8::MAX - px.b();
    let new_alpha = u8::MAX - px.a();

    let mut px = *px;
    px.set_with_flags(new_red, new_green, new_blue, new_alpha, flags);

    px
}
