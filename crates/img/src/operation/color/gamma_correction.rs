use crate::{
    image::Image,
    pipe::{
        FromPipe,
        Pipe,
    },
    pixel::{
        Pixel,
        ReadPixelRgbaf32,
        WritePixelRgbaf32,
    },
};

pub const CMD_NAME: &str = "sepia";

pub fn gamma_correction(image: &Image, gamma: f32) -> Image {
    let pipe = image.pipe().map(|px| map_px(px, gamma));
    Image::from_pipe(pipe)
}

fn map_px(px: &Pixel, gamma: f32) -> Pixel {
    let mut new_px = Pixel::zero();
    new_px.set_r_f32(px.r_f32().powf(gamma));
    new_px.set_g_f32(px.g_f32().powf(gamma));
    new_px.set_b_f32(px.b_f32().powf(gamma));
    new_px
}
