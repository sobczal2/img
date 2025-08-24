#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use thiserror::Error;

use crate::{
    collections::tracking_set::TrackingSet,
    image::Image,
    pixel::{Pixel, PixelMut},
};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum Error {
    #[error("radius too big for given image")]
    RadiusTooBig,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn median_blur(image: &Image, radius: usize) -> Result<Image> {
    validate(image, radius)?;

    let diamater = radius * 2 + 1;
    let mut new_image =
        Image::empty((image.size().0 - diamater + 1, image.size().1 - diamater + 1));

    // TODO: consider switching dimensions since here most
    // nested access occurs on y value in this implementation
    new_image.rows_mut().for_each(|(y, row)| {
        let mut sets = init_sets(image, diamater, y);
        row.for_each(|(x, mut px)| {
            process_pixel((x, y), &mut px, image, radius, &mut sets);
        });
    });

    Ok(new_image)
}

#[cfg(feature = "parallel")]
pub fn median_blur_par(image: &Image, radius: usize) -> Result<Image> {
    validate(image, radius)?;

    let diamater = radius * 2 + 1;
    let mut new_image =
        Image::empty((image.size().0 - diamater + 1, image.size().1 - diamater + 1));

    // TODO: consider switching dimensions since here most
    // nested access occurs on y value in this implementation
    new_image.rows_mut().par_bridge().for_each(|(y, row)| {
        let mut sets = init_sets(image, diamater, y);
        row.for_each(|(x, mut px)| {
            process_pixel((x, y), &mut px, image, radius, &mut sets);
        });
    });

    Ok(new_image)
}

fn validate(image: &Image, radius: usize) -> Result<()> {
    if image.size().0 < radius * 2 + 1 || image.size().1 < radius * 2 + 1 {
        return Err(Error::RadiusTooBig);
    }

    Ok(())
}

struct MedianSets {
    r: TrackingSet<u8>,
    g: TrackingSet<u8>,
    b: TrackingSet<u8>,
}

impl MedianSets {
    fn pop(&mut self, count: usize) {
        (0..count).for_each(|_| {
            self.r.pop();
            self.g.pop();
            self.b.pop();
        });
    }

    fn push(&mut self, px: Pixel) {
        self.r.push(px.r());
        self.g.push(px.g());
        self.b.push(px.b());
    }
}

/// fill out sets withinital data of an image.
/// This does it like for "imaginary" pixel 1 to the left
/// from the starting pixel.
/// this way we can iterate over all without special checks
fn init_sets(image: &Image, diameter: usize, row_y: usize) -> MedianSets {
    let mut r = TrackingSet::new();
    let mut g = TrackingSet::new();
    let mut b = TrackingSet::new();

    (0..diameter).for_each(|_| {
        r.push(0);
        g.push(0);
        b.push(0);
    });

    (0..diameter).for_each(|y| {
        (0..diameter - 1).for_each(|x| {
            let px = unsafe { image.pixel_unchecked((x, y + row_y)) };
            r.push(px.r());
            g.push(px.g());
            b.push(px.b());
        });
    });

    MedianSets { r, g, b }
}

fn process_pixel(
    xy: (usize, usize),
    px: &mut PixelMut,
    original_image: &Image,
    radius: usize,
    sets: &mut MedianSets,
) {
    let (x, y) = xy;
    let diamater = radius * 2 + 1;
    sets.pop(diamater);
    (0..diamater).for_each(|c| {
        let y = c + y;
        let px = unsafe { original_image.pixel_unchecked((x, y)) };
        sets.push(px);
    });

    let new_r = *sets.r.mid().unwrap();
    let new_g = *sets.g.mid().unwrap();
    let new_b = *sets.b.mid().unwrap();

    px.set_r(new_r);
    px.set_g(new_g);
    px.set_b(new_b);
    px.set_a(unsafe { original_image.pixel_unchecked(xy).a() });
}
