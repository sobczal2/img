#[cfg(feature = "parallel")]
use rayon::iter::{ParallelBridge, ParallelIterator};

use std::{
    f32::consts::{E, PI},
    num::NonZero,
};

use thiserror::Error;

use crate::{
    error::{IndexResult, OutOfBoundsError},
    image::Image,
    pixel::PixelMut,
    primitives::{point::Point, size::Size},
};

/// Error returned by mean_blur function
#[derive(Debug, Error)]
pub enum Error {
    #[error("radius too big for given image")]
    RadiusTooBig,
    #[error("invalid sigma - has to be positive")]
    InvalidSigma,
}

pub type Result<T> = std::result::Result<T, Error>;

/// perform mean blur on an image not in place
/// this reduces size of an image by radius * 2 times
/// so to receive image of an original size you should pad it
pub fn gaussian_blur(image: &Image, radius: usize, sigma: f32) -> Result<Image> {
    validate(image, radius, sigma)?;

    let kernel = GaussianKernel::new(radius, sigma);
    let diameter = radius * 2 + 1;
    let mut new_image = Image::empty(
        Size::from_usize(
            image.size().width() - diameter + 1,
            image.size().height() - diameter + 1,
        )
        .unwrap(),
    );

    new_image.rows_mut().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            process_pixel(Point::new(x, y), &mut px, image, &kernel);
        });
    });

    Ok(new_image)
}

#[cfg(feature = "parallel")]
pub fn gaussian_blur_par(image: &Image, radius: usize, sigma: f32) -> Result<Image> {
    validate(image, radius, sigma)?;

    let kernel = GaussianKernel::new(radius, sigma);
    let diameter = radius * 2 + 1;
    let mut new_image =
        Image::empty((image.size().0 - diameter + 1, image.size().1 - diameter + 1));

    new_image.rows_mut().par_bridge().for_each(|(y, row)| {
        row.for_each(|(x, mut px)| {
            process_pixel((x, y), &mut px, image, &kernel);
        });
    });

    Ok(new_image)
}

fn validate(image: &Image, radius: usize, sigma: f32) -> Result<()> {
    if image.size().width() < radius * 2 + 1 || image.size().height() < radius * 2 + 1 {
        return Err(Error::RadiusTooBig);
    }

    if sigma <= 0f32 {
        return Err(Error::InvalidSigma);
    }

    Ok(())
}

fn process_pixel(point: Point, px: &mut PixelMut, original_image: &Image, kernel: &GaussianKernel) {
    let diameter = kernel.size().width();
    let radius = kernel.size().width() / 2;
    // TODO: fix this unwrap
    let radius_i32: i32 = radius.try_into().unwrap();
    let sum = (0..diameter)
        .flat_map(|k_y| {
            (0..diameter).map(move |k_x| {
                let new_px = unsafe {
                    original_image.pixel_unchecked(Point::new(point.x() + k_x, point.y() + k_y))
                };
                let offset = (k_x as i32 - radius_i32, k_y as i32 - radius_i32);
                let kernel_value = unsafe { kernel.value_unchecked(offset) };
                (
                    new_px.r() as f32 * kernel_value,
                    new_px.g() as f32 * kernel_value,
                    new_px.b() as f32 * kernel_value,
                )
            })
        })
        .fold((0f32, 0f32, 0f32), |(acc_r, acc_g, acc_b), (r, g, b)| {
            (acc_r + r, acc_g + g, acc_b + b)
        });

    px.set_r(sum.0 as u8);
    px.set_g(sum.1 as u8);
    px.set_b(sum.2 as u8);
    px.set_a(unsafe { original_image.pixel_unchecked(point).a() });
}

#[derive(Debug)]
pub struct GaussianKernel {
    values: Box<[f32]>,
    radius: usize,
}

impl GaussianKernel {
    /// create a gaussian kernel using radius and sigma
    pub fn new(radius: usize, sigma: f32) -> GaussianKernel {
        let diameter = radius * 2 + 1;
        let mut values = vec![0f32; diameter * diameter].into_boxed_slice();

        // SAFETY: diameter always >= 1
        let diameter = NonZero::new(diameter).unwrap();
        let size = Size::new(diameter, diameter);

        values.iter_mut().enumerate().for_each(|(index, value)| {
            // SAFETY: we iterate over values which have size equal to size variable
            let point = unsafe { Point::from_index_unchecked(index, size) };
            let offset = (
                point.x() as i32 - radius as i32,
                point.y() as i32 - radius as i32,
            );
            *value = gaussian_fn(offset, sigma);
        });

        let sum = values.iter().sum::<f32>();
        let sum_inv = 1f32 / sum;

        values.iter_mut().for_each(|value| *value *= sum_inv);

        GaussianKernel { values, radius }
    }

    /// get kernel size
    pub fn size(&self) -> Size {
        let diameter = self.radius * 2 + 1;

        // SAFETY: diameter always >= 1
        Size::from_usize(diameter, diameter).unwrap()
    }

    /// get kernel value given the offset
    pub fn value(&self, offset: (i32, i32)) -> IndexResult<f32> {
        let point = offset_to_point(offset, self.radius)?;
        let idx = unsafe { point.to_index_unchecked(self.size()) };
        if idx > self.values.len() {
            return Err(OutOfBoundsError);
        }

        Ok(self.values[idx])
    }

    /// get kernel value given the offset
    /// without checking bounds
    ///
    /// # Safety
    ///
    /// this should be called only using valid x and y
    pub unsafe fn value_unchecked(&self, offset: (i32, i32)) -> f32 {
        let point = offset_to_point(offset, self.radius).unwrap();
        let idx = unsafe { point.to_index_unchecked(self.size()) };
        self.values[idx]
    }
}

fn offset_to_point(offset: (i32, i32), radius: usize) -> IndexResult<Point> {
    if offset.0.abs() > radius as i32 || offset.1.abs() > radius as i32 {
        return Err(OutOfBoundsError);
    }

    Ok(Point::new(
        (offset.0 + radius as i32)
            .try_into()
            .map_err(|_| OutOfBoundsError)?,
        (offset.1 + radius as i32)
            .try_into()
            .map_err(|_| OutOfBoundsError)?,
    ))
}

fn gaussian_fn(offset: (i32, i32), sigma: f32) -> f32 {
    let sigma_2 = sigma * sigma;
    let x_2 = (offset.0 * offset.0) as f32;
    let y_2 = (offset.1 * offset.1) as f32;

    (1f32 / (2f32 * PI * sigma_2)) * E.powf(-(x_2 + y_2) / (2f32 * sigma_2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gaussian_fn() {
        const PRECISION: f32 = 1e-8;

        assert!((gaussian_fn((0, 0), 1f32) - 0.15915494).abs() <= PRECISION);
        assert!((gaussian_fn((1, 0), 1f32) - 0.09653235).abs() <= PRECISION);
        assert!((gaussian_fn((1, 1), 1f32) - 0.05854983).abs() <= PRECISION);
        assert!((gaussian_fn((0, 0), 2f32) - 0.03978873).abs() <= PRECISION);

        assert_eq!(gaussian_fn((1, 0), 1f32), gaussian_fn((0, 1), 1f32));
        assert_eq!(gaussian_fn((1, 0), 2f32), gaussian_fn((0, 1), 2f32));
        assert_eq!(gaussian_fn((1, 0), 3f32), gaussian_fn((0, 1), 3f32));
    }

    #[test]
    fn test_offset_to_point() {
        assert_eq!(offset_to_point((0, 0), 2), Ok(Point::new(2, 2)));
        assert_eq!(offset_to_point((-2, -2), 2), Ok(Point::new(0, 0)));
        assert_eq!(offset_to_point((2, 2), 2), Ok(Point::new(4, 4)));
        assert_eq!(offset_to_point((3, 2), 2), Err(OutOfBoundsError));
        assert_eq!(offset_to_point((-3, -2), 2), Err(OutOfBoundsError));
    }
}
