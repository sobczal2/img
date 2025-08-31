use crate::{
    image::Image,
    pipe::{
        FromPipe,
        Pipe,
    },
    pixel::{
        Pixel,
        PixelFlags,
    },
};

pub fn grayscale_pipe<S>(source: S, flags: PixelFlags) -> impl Pipe<Item = Pixel>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    source.map(move |px| map_px(px, flags))
}

fn map_px(px: impl AsRef<Pixel>, flags: PixelFlags) -> Pixel {
    let px = px.as_ref();
    let value = 0.299 * px.r() as f32 + 0.587 * px.g() as f32 + 0.214 * px.b() as f32;
    let value = value as u8;

    let mut px = *px;
    px.set_with_flags(value, value, value, value, flags);

    px
}

pub fn grayscale(image: &Image, flags: PixelFlags) -> Image {
    let pipe = grayscale_pipe(image.pipe(), flags);
    Image::from_pipe(pipe)
}

#[cfg(feature = "parallel")]
pub fn grayscale_par(image: &Image, flags: PixelFlags) -> Image {
    use crate::pipe::FromPipePar;

    let pipe = grayscale_pipe(image.pipe(), flags);
    Image::from_pipe_par(pipe)
}
