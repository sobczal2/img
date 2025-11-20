use std::f32::consts::PI;
#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;

use itertools::Itertools;
use thiserror::Error;

use crate::{
    component::{
        kernel::{
            gaussian::{GaussianKernel, GaussianKernelCreationError}, sobel::{
                Gradient,
                SobelKernel,
            }, Kernel
        },
        lens::border::value_border,
        primitive::{
            Margin,
            Offset,
            Point,
        },
    },
    error::IndexResult,
    image::Image,
    lens::{
        kernel::KernelLensCreationError, overlay::OverlayLensCreationError, FromLens, Lens
    },
    pixel::{
        ChannelFlags,
        Pixel,
    },
};

pub struct CannyLensOptions {
    pub gaussian_radius: usize,
    pub gaussian_sigma: f32,
}

impl Default for CannyLensOptions {
    fn default() -> Self {
        Self {
            gaussian_radius: 2,
            gaussian_sigma: 2.0,
        }
    }
}

impl CannyLensOptions {
    pub fn builder() -> CannyLensOptionsBuilder {
        CannyLensOptionsBuilder::new()
    }
}

pub struct CannyLensOptionsBuilder {
    options: CannyLensOptions,
}

impl CannyLensOptionsBuilder {
    pub fn new() -> Self {
        Self {
            options: CannyLensOptions::default(),
        }
    }

    pub fn gaussian_radius(mut self, radius: usize) -> Self {
        self.options.gaussian_radius = radius;
        self
    }

    pub fn gaussian_sigma(mut self, sigma: f32) -> Self {
        self.options.gaussian_sigma = sigma;
        self
    }

    pub fn build(self) -> CannyLensOptions {
        self.options
    }
}

#[derive(Debug, Error)]
pub enum CannyCreationError {
    #[error("Intermediate lens is too big")]
    IntermediateLensTooBig,
    #[error("gaussian radius too big")]
    GaussianRadiusTooBig,
    #[error("gaussian kernel creation error: {0}")]
    GaussianKernelCreation(#[from] GaussianKernelCreationError),
    #[error("gaussian kernel lens creation error: {0}")]
    KernelLensCreation(#[from] KernelLensCreationError),
}

pub type CannyCreationResult<T> = std::result::Result<T, CannyCreationError>;

pub fn canny_lens<S>(source: S, options: CannyLensOptions) -> CannyCreationResult<impl Lens<Item = Pixel>>
where
    S: Lens<Item = Pixel> + Clone,
{
    // SAFETY: Margin::unified only fails if argument is >= DIMENSION_MAX
    let margin = Margin::unified(options.gaussian_radius).map_err(|_| CannyCreationError::GaussianRadiusTooBig)?;
    let lens = value_border(
        source,
        margin,
        Pixel::zero(),
    )
    .map_err(|e| match e {
        OverlayLensCreationError::OverlayTooBig => CannyCreationError::IntermediateLensTooBig,
        _ => unreachable!("Unexpected error in value_border")
    })?;

    Ok(lens.kernel(
        GaussianKernel::new(
            margin,
            options.gaussian_sigma,
            ChannelFlags::RGB,
        )?
    )?
    .materialize()
    .split4(
        |s| single_channel_lens(s.map(|p| p.r())),
        |s| single_channel_lens(s.map(|p| p.g())),
        |s| single_channel_lens(s.map(|p| p.b())),
        |s| s.map(|p| p.a()),
    )
    .map(|(r, g, b, a)| Pixel::new([r, g, b, a])))
}

#[cfg(feature = "parallel")]
pub fn canny_lens_par<S>(source: S, options: CannyLensOptions, threads: NonZeroUsize) -> CannyCreationResult<impl Lens<Item = Pixel>>
where
    S: Lens<Item = Pixel> + Clone + Send + Sync,
{
    // SAFETY: Margin::unified only fails if argument is >= DIMENSION_MAX
    let margin = Margin::unified(options.gaussian_radius).map_err(|_| CannyCreationError::GaussianRadiusTooBig)?;
    let lens = value_border(
        source,
        margin,
        Pixel::zero(),
    )
    .map_err(|e| match e {
        OverlayLensCreationError::OverlayTooBig => CannyCreationError::IntermediateLensTooBig,
        _ => unreachable!("Unexpected error in value_border")
    })?;

    Ok(lens.kernel(
        GaussianKernel::new(
            margin,
            options.gaussian_sigma,
            ChannelFlags::RGB,
        )?
    )?
    .materialize_par(threads)
    .split4(
        |s| single_channel_lens(s.map(|p| p.r())),
        |s| single_channel_lens(s.map(|p| p.g())),
        |s| single_channel_lens(s.map(|p| p.b())),
        |s| s.map(|p| p.a()),
    )
    .map(|(r, g, b, a)| Pixel::new([r, g, b, a])))
}

pub fn canny(image: &Image, options: CannyLensOptions) -> CannyCreationResult<Image> {
    let lens = canny_lens(image.lens().cloned(), options)?;
    Ok(Image::from_lens(lens))
}

#[cfg(feature = "parallel")]
pub fn canny_par(image: &Image, options: CannyLensOptions, threads: NonZeroUsize) -> CannyCreationResult<Image> {
    use crate::lens::FromLensPar;

    let lens = canny_lens_par(image.lens().cloned(), options, threads)?;
    Ok(Image::from_lens_par(lens, threads))
}

fn single_channel_lens<S>(source: S) -> impl Lens<Item = u8>
where
    S: Lens<Item = u8>,
{
    let lens =
        // SAFETY: 1x1x1x1 margin creation should never fail
        value_border(source, Margin::unified(1).expect("unexpected error in Margin::unified"), 0u8)
        // SAFETY: Only case where this fails is if lens size exceeds DIMENSION_MAX, which
        // is not possible here due to previous checks
            .expect("unexpected error in value_border");

    // SAFETY: kernel expects at least 3x3 image which is guaranteed by adding the margin above
    let lens = lens.kernel(SobelKernel::new()).expect("unexpected error in SobelKernel::new");
    let lens = value_border(
        lens,
        // SAFETY: 1x1x1x1 margin creation should never fail
        Margin::unified(1).expect("unexpected error in Margin::unified"),
        Default::default(),
    )
    // SAFETY: Only case where this fails is if lens size exceeds DIMENSION_MAX, which
    // is not possible here due to previous checks
    .expect("unexpected error in value_border");
    let lens = non_maximum_suppression_lens(lens);
    let lens =
        value_border(lens, Margin::unified(1).expect("unexpected error in Margin::unified"), 0f32)
    // SAFETY: Only case where this fails is if lens size exceeds DIMENSION_MAX, which
    // is not possible here due to previous checks
            .expect("unexpected error in value_border");
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
    let size = source
        .size()
        .shrink_by_margin(Margin::unified(1).expect("unexpected error in Margin::unified"))
        .expect("TODO");
    source.map(|g| (g.magnitude(), g.direction())).remap(
        |s, p| {
            let p = p.translate(Offset::new(1, 1).expect("TODO")).expect("TODO");
            let gradient_a = s.look(p).expect("TODO");
            let direction = GradientDirection::from_angle(gradient_a.1);

            let gradient_b = match direction {
                GradientDirection::Horizontal => {
                    s.look(Point::new(p.x() + 1, p.y()).expect("TODO")).expect("TODO")
                }
                GradientDirection::Vertical => {
                    s.look(Point::new(p.x(), p.y() + 1).expect("TODO")).expect("TODO")
                }
            };

            let gradient_c = match direction {
                GradientDirection::Horizontal => {
                    s.look(Point::new(p.x() - 1, p.y()).expect("TODO")).expect("TODO")
                }
                GradientDirection::Vertical => {
                    s.look(Point::new(p.x(), p.y() - 1).expect("TODO")).expect("TODO")
                }
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
    fn evaluate<P>(&self, lens: &P, point: Point) -> IndexResult<u8>
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
            .map(|offset| point.translate(offset.expect("TODO")).expect("TODO"))
            .map(|point| lens.look(point).expect("TODO"))
            .any(|value| value > self.max);

        if neighbor_exists { Ok(255u8) } else { Ok(0u8) }
    }

    fn margin(&self) -> Margin {
        Margin::unified(1).expect("unexpected error in Margin::unified")
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
