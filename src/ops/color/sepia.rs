#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{image::Image, pixel::PixelMut};

/// Transform image to grayscale in place
pub fn sepia(image: &mut Image) {
    #[cfg(feature = "parallel")]
    image.pixels_mut().par_bridge().for_each(px_to_sepia);

    #[cfg(not(feature = "parallel"))]
    image.pixels_mut().for_each(px_to_sepia);
}

fn px_to_sepia(mut px: PixelMut) {
    let new_red = 0.393 * px.r() as f32 + 0.769 * px.g() as f32 + 0.189 * px.b() as f32;
    let new_green = 0.349 * px.r() as f32 + 0.686 * px.g() as f32 + 0.168 * px.b() as f32;
    let new_blue = 0.272 * px.r() as f32 + 0.534 * px.g() as f32 + 0.131 * px.b() as f32;

    let new_red = new_red.clamp(0f32, 255f32) as u8;
    let new_green = new_green.clamp(0f32, 255f32) as u8;
    let new_blue = new_blue.clamp(0f32, 255f32) as u8;

    px.set_r(new_red);
    px.set_g(new_green);
    px.set_b(new_blue);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pixel::{Pixel, PixelMut};

    #[test]
    fn test_px_to_sepia() {
        let data = &mut [10, 20, 30, 50];
        {
            let px = PixelMut::new(data);
            px_to_sepia(px);
        }

        assert_eq!(Pixel::new(data), Pixel::new(&[24, 22, 17, 50]));
    }
}
