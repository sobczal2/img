use crate::{
    image::Image,
    pixel::{PixelMut, ReadPixelRgbaf32, WritePixelRgbaf32},
};

pub const CMD_NAME: &str = "sepia";

pub fn gamma_correction(image: &mut Image, gamma: f32) {
    image
        .pixels_mut()
        .for_each(|mut px| px_gamma_correction(&mut px, gamma));
}

fn px_gamma_correction(px: &mut PixelMut, gamma: f32) {
    px.set_r_f32(px.r_f32().powf(gamma));
    px.set_g_f32(px.g_f32().powf(gamma));
    px.set_b_f32(px.b_f32().powf(gamma));
}
