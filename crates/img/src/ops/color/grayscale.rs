#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{image::Image, pixel::PixelMut};

/// Transform image to grayscale in place
pub fn grayscale(image: &mut Image) {
    image.pixels_mut().for_each(px_to_grayscale);
}

#[cfg(feature = "parallel")]
pub fn grayscale_par(image: &mut Image) {
    image.pixels_mut().par_bridge().for_each(px_to_grayscale);
}

fn px_to_grayscale(mut px: PixelMut) {
    let value = 0.299 * px.r() as f32 + 0.587 * px.g() as f32 + 0.214 * px.b() as f32;
    let value = value as u8;

    px.set_r(value);
    px.set_g(value);
    px.set_b(value);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pixel::{Pixel, PixelMut};

    #[test]
    fn test_px_to_grayscale() {
        let data = &mut [10, 20, 30, 50];
        {
            let px = PixelMut::new(data);
            px_to_grayscale(px);
        }

        assert_eq!(Pixel::new(data), Pixel::new(&[21, 21, 21, 50]));
    }
}
