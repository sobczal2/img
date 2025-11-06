use std::f32::consts::PI;
#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use itertools::Itertools;

use crate::{
    component::{
        kernel::{
            Kernel,
            gaussian::GaussianKernel,
            sobel::{
                Gradient,
                SobelKernel,
            },
        },
        lens::border::value_border,
        primitive::{
            Margin,
            Offset,
            Point,
            Size,
        },
    },
    error::IndexResult,
    image::Image,
    lens::{
        FromLens,
        Lens,
    },
    pixel::{
        ChannelFlags,
        Pixel,
    },
};

// #[derive(Debug, Error)]
// pub enum CreationError {
//     #[error("kernel creation error: {0}")]
//     KernelCreationError(#[from] kernel::CreationError),
// }
//
// pub type CreationResult<T> = std::result::Result<T, CreationError>;

pub fn canny_lens<S>(source: S) -> impl Lens<Item = Pixel>
where
    S: Lens<Item = Pixel> + Clone,
{
    let lens = value_border(source, Margin::unified(2), Pixel::zero()).expect("TODO");

    // SAFETY: `Size::from_radius(2)` is always successful.
    lens.kernel(
        GaussianKernel::new(Size::from_radius(2).expect("unexpected error from Size::from_radius"), 2f32, ChannelFlags::RGB).expect("TODO"),
    )
    .expect("TODO")
    .materialize()
    .split4(
        |s| single_channel_lens(s.map(|p| p.r())),
        |s| single_channel_lens(s.map(|p| p.g())),
        |s| single_channel_lens(s.map(|p| p.b())),
        |s| s.map(|p| p.a()),
    )
    .map(|(r, g, b, a)| Pixel::new([r, g, b, a]))
}

#[cfg(feature = "parallel")]
pub fn canny_lens_par<S>(source: S, threads: NonZeroUsize) -> impl Lens<Item = Pixel>
where
    S: Lens<Item = Pixel> + Clone + Send + Sync,
{
    let lens = value_border(source, Margin::unified(2), Pixel::zero()).expect("TODO");

    // SAFETY: `Size::from_radius(2)` is always successful.
    lens.kernel(
        GaussianKernel::new(Size::from_radius(2).expect("unexpected error from Size::from_radius"), 2f32, ChannelFlags::RGB).expect("TODO"),
    )
    .expect("TODO")
    .materialize_par(threads)
    .split4(
        |s| single_channel_lens(s.map(|p| p.r())),
        |s| single_channel_lens(s.map(|p| p.g())),
        |s| single_channel_lens(s.map(|p| p.b())),
        |s| s.map(|p| p.a()),
    )
    .map(|(r, g, b, a)| Pixel::new([r, g, b, a]))
}

pub fn canny(image: &Image) -> Image {
    let lens = canny_lens(image.lens().cloned());
    Image::from_lens(lens)
}

#[cfg(feature = "parallel")]
pub fn canny_par(image: &Image, threads: NonZeroUsize) -> Image {
    use crate::lens::FromLensPar;

    let lens = canny_lens_par(image.lens().cloned(), threads);
    Image::from_lens_par(lens, threads)
}

fn single_channel_lens<S>(source: S) -> impl Lens<Item = u8>
where
    S: Lens<Item = u8>,
{
    let lens = value_border(source, Margin::unified(1), 0u8).expect("TODO");
    let lens = lens.kernel(SobelKernel::new()).expect("TODO");
    let lens = value_border(lens, Margin::unified(1), Default::default()).expect("TODO");
    let lens = non_maximum_suppression_lens(lens);
    let lens = value_border(lens, Margin::unified(1), 0f32).expect("TODO");
    hysteresis_thresholding_lens(lens)
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
    let size = source.size().shrink_by_margin(Margin::unified(1)).expect("TODO");
    source.map(|g| (g.magnitude(), g.direction())).remap(
        |s, p| {
            let p = p.translate(Offset::new(1, 1)).expect("TODO");
            let gradient_a = s.look(p).expect("TODO");
            let direction = GradientDirection::from_angle(gradient_a.1);

            let gradient_b = match direction {
                GradientDirection::Horizontal => s.look(Point::new(p.x() + 1, p.y())).expect("TODO"),
                GradientDirection::Vertical => s.look(Point::new(p.x(), p.y() + 1)).expect("TODO"),
            };

            let gradient_c = match direction {
                GradientDirection::Horizontal => s.look(Point::new(p.x() - 1, p.y())).expect("TODO"),
                GradientDirection::Vertical => s.look(Point::new(p.x(), p.y() - 1)).expect("TODO"),
            };

            if gradient_a.0 > gradient_b.0 && gradient_a.0 > gradient_c.0 {
                Ok(gradient_a.0)
            } else {
                Ok(0f32)
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
            .map(|offset| point.translate(offset).expect("TODO"))
            .map(|point| lens.look(point).expect("TODO"))
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

    source.kernel(HysteresisThresholdingKernel { min, max }).expect("TODO")
}
