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

pub fn sepia_pipe<S>(source: S, flags: PixelFlags) -> impl Pipe<Item = Pixel>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    source.map(move |px| map_px(px, flags))
}

pub fn sepia(image: &Image, flags: PixelFlags) -> Image {
    let pipe = sepia_pipe(image.pipe(), flags);
    Image::from_pipe(pipe)
}

#[cfg(feature = "parallel")]
pub fn sepia_par(image: &Image, flags: PixelFlags) -> Image {
    use crate::pipe::FromPipePar;

    let pipe = sepia_pipe(image.pipe(), flags);
    Image::from_pipe_par(pipe)
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
