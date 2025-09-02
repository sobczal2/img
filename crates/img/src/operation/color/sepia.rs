use crate::{
    image::Image,
    lens::{
        FromLens,
        Lens,
    },
    pixel::{
        Pixel,
        PixelFlags,
    },
};

pub fn sepia_lens<S>(source: S, flags: PixelFlags) -> impl Lens<Item = Pixel>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    source.map(move |px| map_px(px, flags))
}

pub fn sepia(image: &Image, flags: PixelFlags) -> Image {
    let lens = sepia_lens(image.lens(), flags);
    Image::from_lens(lens)
}

#[cfg(feature = "parallel")]
pub fn sepia_par(image: &Image, flags: PixelFlags) -> Image {
    use crate::lens::FromLensPar;

    let lens = sepia_lens(image.lens(), flags);
    Image::from_lens_par(lens)
}

fn map_px(px: impl AsRef<Pixel>, flags: PixelFlags) -> Pixel {
    let px = px.as_ref();

    let new_red = 0.393 * px.r() as f32 + 0.769 * px.g() as f32 + 0.189 * px.b() as f32;
    let new_green = 0.349 * px.r() as f32 + 0.686 * px.g() as f32 + 0.168 * px.b() as f32;
    let new_blue = 0.272 * px.r() as f32 + 0.534 * px.g() as f32 + 0.131 * px.b() as f32;

    let new_red = new_red.clamp(0f32, 255f32) as u8;
    let new_green = new_green.clamp(0f32, 255f32) as u8;
    let new_blue = new_blue.clamp(0f32, 255f32) as u8;

    let mut px = *px;
    px.set_with_flags(new_red, new_green, new_blue, px.a(), flags);

    px
}
