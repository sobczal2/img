use crate::{
    image::Image,
    pipe::{
        FromPipe,
        Pipe,
    },
    pixel::Pixel,
};

pub fn sepia(image: &Image) -> Image {
    let pipe = image.pipe().map(map_px);
    Image::from_pipe(pipe)
}

fn map_px(px: &Pixel) -> Pixel {
    let new_red = 0.393 * px.r() as f32 + 0.769 * px.g() as f32 + 0.189 * px.b() as f32;
    let new_green = 0.349 * px.r() as f32 + 0.686 * px.g() as f32 + 0.168 * px.b() as f32;
    let new_blue = 0.272 * px.r() as f32 + 0.534 * px.g() as f32 + 0.131 * px.b() as f32;

    let new_red = new_red.clamp(0f32, 255f32) as u8;
    let new_green = new_green.clamp(0f32, 255f32) as u8;
    let new_blue = new_blue.clamp(0f32, 255f32) as u8;

    Pixel::new([new_red, new_green, new_blue, px.a()])
}
