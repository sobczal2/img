use std::f32::consts::PI;

use itertools::Itertools;
use thiserror::Error;

use crate::{
    component::kernel::{
        Kernel,
        gaussian::GaussianKernel,
        sobel::{
            Gradient,
            SobelKernel,
        },
    },
    error::IndexResult,
    image::Image,
    lens::{
        FromLens,
        Lens,
        border::BorderFill,
        image::PixelLens,
        kernel,
    },
    pixel::{
        Pixel,
        PixelFlags,
    },
    primitive::{
        margin::Margin,
        offset::Offset,
        point::Point,
        size::Size,
    },
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("kernel creation error: {0}")]
    KernelCreationError(#[from] kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

pub fn canny_lens<S>(source: S) -> Result<impl Lens<Item = Pixel>, CreationError>
where
    S: Lens,
    S::Item: AsRef<Pixel>,
{
    let lens = source
        .border(Margin::unified(2), BorderFill::PickZero)
        .kernel(GaussianKernel::new(Size::from_radius(2), 2f32, PixelFlags::RGB).unwrap())?
        .colors(
            single_channel_lens,
            single_channel_lens,
            single_channel_lens,
            CreationResult::Ok,
        )?;

    Ok(lens)
}

#[cfg(feature = "parallel")]
pub fn canny_lens_par<S>(source: S) -> Result<impl Lens<Item = Pixel>, CreationError>
where
    S: Lens + Send + Sync,
    S::Item: AsRef<Pixel>,
{
    let lens = source
        .border(Margin::unified(2), BorderFill::PickZero)
        .kernel(GaussianKernel::new(Size::from_radius(2), 2f32, PixelFlags::RGB).unwrap())?
        .colors_par(
            single_channel_lens,
            single_channel_lens,
            single_channel_lens,
            CreationResult::Ok,
        )?;

    Ok(lens)
}

pub fn canny(image: &Image) -> Image {
    let lens = canny_lens(image.lens()).unwrap();
    Image::from_lens(lens)
}

#[cfg(feature = "parallel")]
pub fn canny_par(image: &Image) -> Image {
    use crate::lens::FromLensPar;

    let lens = canny_lens_par(image.lens()).unwrap();
    Image::from_lens_par(lens)
}

fn single_channel_lens<S>(source: S) -> Result<impl Lens<Item = u8>, CreationError>
where
    S: Lens<Item = u8>,
{
    let lens = source.border(Margin::unified(1), BorderFill::PickZero);
    let lens = lens.kernel(SobelKernel::new())?;
    let lens = lens.border(Margin::unified(1), BorderFill::PickZero);
    let lens = non_maximum_suppression_lens(lens);
    let lens = lens.border(Margin::unified(1), BorderFill::PickZero);
    let lens = hysteresis_thresholding_lens(lens);

    Ok(lens)
}

enum GradientDirection {
    Horizontal,
    Vertical,
}

impl GradientDirection {
    fn from_angle(angle: f32) -> GradientDirection {
        let mut angle = angle.abs() % PI;
        if angle > PI / 2f32 {
            angle = PI - angle;
        }

        if angle < PI / 4f32 { GradientDirection::Horizontal } else { GradientDirection::Vertical }
    }
}

fn non_maximum_suppression_lens<S>(source: S) -> impl Lens<Item = f32>
where
    S: Lens<Item = Gradient>,
{
    let size = source.size().apply_margin(Margin::unified(1)).unwrap();
    source.map(|g| (g.magnitude(), g.direction())).remap(
        |s, p| {
            let p = p.translate(Offset::new(1, 1)).unwrap();
            let gradient_a = s.look(p).unwrap();
            let direction = GradientDirection::from_angle(gradient_a.1);

            let gradient_b = match direction {
                GradientDirection::Horizontal => s.look(Point::new(p.x() + 1, p.y())).unwrap(),
                GradientDirection::Vertical => s.look(Point::new(p.x(), p.y() + 1)).unwrap(),
            };

            let gradient_c = match direction {
                GradientDirection::Horizontal => s.look(Point::new(p.x() - 1, p.y())).unwrap(),
                GradientDirection::Vertical => s.look(Point::new(p.x(), p.y() - 1)).unwrap(),
            };

            if gradient_a.0 > gradient_b.0 && gradient_a.0 > gradient_c.0 {
                gradient_a.0
            } else {
                0f32
            }
        },
        size,
    )
}

struct HysteresisThresholdingKernel {
    min: f32,
    max: f32,
}

impl Kernel<f32, u8> for HysteresisThresholdingKernel {
    fn apply<P>(&self, lens: &P, point: Point) -> IndexResult<u8>
    where
        P: Lens<Item = f32>,
    {
        let v = lens.look(point)?;

        if v > self.max {
            return Ok(255u8);
        }

        if v < self.min {
            return Ok(0u8);
        }

        let neighbor_exists = (-1..=1)
            .cartesian_product(-1..=1)
            .map(|(x, y)| Offset::new(x, y))
            .map(|offset| point.translate(offset).unwrap())
            .map(|point| lens.look(point).unwrap())
            .any(|value| value > self.max);

        if neighbor_exists { Ok(255u8) } else { Ok(0u8) }
    }

    fn margin(&self) -> Margin {
        Margin::unified(1)
    }
}

fn hysteresis_thresholding_lens<S>(source: S) -> impl Lens<Item = u8>
where
    S: Lens<Item = f32>,
{
    let min = 10f32;
    let max = 20f32;

    source.kernel(HysteresisThresholdingKernel { min, max }).unwrap()
}
