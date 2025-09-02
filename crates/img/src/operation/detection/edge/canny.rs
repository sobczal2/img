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
    pipe::{
        FromPipe,
        Pipe,
        image::PixelPipe,
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

pub fn canny_pipe<S>(source: S) -> Result<impl Pipe<Item = Pixel>, CreationError>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    let pipe = source
        .kernel(GaussianKernel::new(Size::from_radius(2), 2f32, PixelFlags::RGB).unwrap())?
        .colors(
            single_channel_pipe,
            single_channel_pipe,
            single_channel_pipe,
            CreationResult::Ok,
        )?;

    Ok(pipe)
}

pub fn canny(image: &Image) -> Image {
    let pipe = canny_pipe(image.pipe()).unwrap();
    Image::from_pipe(pipe)
}

#[cfg(feature = "parallel")]
pub fn canny_par(image: &Image) -> Image {
    use crate::pipe::FromPipePar;

    let pipe = canny_pipe(image.pipe()).unwrap();
    Image::from_pipe_par(pipe)
}

fn single_channel_pipe<S>(source: S) -> Result<impl Pipe<Item = u8>, CreationError>
where
    S: Pipe<Item = u8>,
{
    let pipe = source.kernel(SobelKernel::new())?;
    let pipe = non_maximum_suppression_pipe(pipe);
    let pipe = hysteresis_thresholding_pipe(pipe);

    Ok(pipe)
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

fn non_maximum_suppression_pipe<S>(source: S) -> impl Pipe<Item = f32>
where
    S: Pipe<Item = Gradient>,
{
    let size = source.size().apply_margin(Margin::unified(1)).unwrap();
    source.map(|g| (g.magnitude(), g.direction())).remap(
        |s, p| {
            let p = p.offset_by(Offset::new(1, 1)).unwrap();
            let gradient_a = s.get(p).unwrap();
            let direction = GradientDirection::from_angle(gradient_a.1);

            let gradient_b = match direction {
                GradientDirection::Horizontal => s.get(Point::new(p.x() + 1, p.y())).unwrap(),
                GradientDirection::Vertical => s.get(Point::new(p.x(), p.y() + 1)).unwrap(),
            };

            let gradient_c = match direction {
                GradientDirection::Horizontal => s.get(Point::new(p.x() - 1, p.y())).unwrap(),
                GradientDirection::Vertical => s.get(Point::new(p.x(), p.y() - 1)).unwrap(),
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
    fn apply<P>(&self, pipe: &P, point: Point) -> IndexResult<u8>
    where
        P: Pipe<Item = f32>,
    {
        let v = pipe.get(point)?;

        if v > self.max {
            return Ok(255u8);
        }

        if v < self.min {
            return Ok(0u8);
        }

        let neighbor_exists = (-1..=1)
            .cartesian_product(-1..=1)
            .map(|(x, y)| Offset::new(x, y))
            .map(|offset| point.offset_by(offset).unwrap())
            .map(|point| pipe.get(point).unwrap())
            .any(|value| value > self.max);

        if neighbor_exists { Ok(255u8) } else { Ok(0u8) }
    }

    fn size(&self) -> Size {
        Size::from_radius(1)
    }
}

fn hysteresis_thresholding_pipe<S>(source: S) -> impl Pipe<Item = u8>
where
    S: Pipe<Item = f32>,
{
    let min = 10f32;
    let max = 20f32;

    source.kernel(HysteresisThresholdingKernel { min, max }).unwrap()
}
