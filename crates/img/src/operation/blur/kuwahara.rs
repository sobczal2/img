#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;
use std::ops::{
    Add,
    Div,
};

use itertools::Itertools;

#[cfg(feature = "parallel")]
use crate::image::Image;
use crate::{
    component::{
        kernel::Kernel,
        primitive::{
            Margin,
            Offset,
            Point,
            Size,
        },
    },
    error::IndexResult,
    lens::{
        FromLens,
        Lens,
    },
    pixel::{
        Pixel,
        hsv::HsvPixel,
    },
};

pub fn kuwahara(image: &Image) -> Image {
    let lens = kuwahara_lens(image.lens().cloned(), 5);
    Image::from_lens(lens)
}

#[cfg(feature = "parallel")]
pub fn kuwahara_par(image: &Image, threads: NonZeroUsize) -> Image {
    use crate::lens::FromLensPar;

    let lens = kuwahara_lens(image.lens().cloned(), 5);
    Image::from_lens_par(lens, threads)
}

// TODO: radius check
pub fn kuwahara_lens<S>(source: S, radius: usize) -> impl Lens<Item = S::Item>
where
    S: Lens<Item = Pixel> + Clone,
{
    source
        .split2(
            |s| s.map(HsvPixel::from).kernel(QuadrantSelectionKernel { radius }).expect("TODO"),
            |s| s,
        )
        .map(|(selected_quadrant, pixel)| MeanCalculationInput { selected_quadrant, pixel })
        .kernel(MeanCalculationKernel { radius })
        .expect("TODO")
}

enum SelectedQuadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct QuadrantSelectionKernel {
    radius: usize,
}

impl Kernel<HsvPixel, SelectedQuadrant> for QuadrantSelectionKernel {
    fn apply<S>(&self, source: &S, point: Point) -> IndexResult<SelectedQuadrant>
    where
        S: Lens<Item = HsvPixel>,
    {
        let quadrant_size = Size::new(self.radius + 1, self.radius + 1).expect("TODO");
        let std_dev_q1 = calculate_std_dev(
            source,
            Point::new(point.x() - self.radius, point.y() - self.radius),
            quadrant_size,
        );
        let std_dev_q2 = calculate_std_dev(
            source,
            Point::new(point.x(), point.y() - self.radius),
            quadrant_size,
        );
        let std_dev_q3 = calculate_std_dev(
            source,
            Point::new(point.x() - self.radius, point.y()),
            quadrant_size,
        );
        let std_dev_q4 = calculate_std_dev(source, Point::new(point.x(), point.y()), quadrant_size);

        let std_devs = [std_dev_q1, std_dev_q2, std_dev_q3, std_dev_q4];
        let min = std_devs.iter().min_by(|a, b| a.partial_cmp(b).expect("TODO")).expect("TODO");

        if *min == std_dev_q1 {
            Ok(SelectedQuadrant::TopLeft)
        } else if *min == std_dev_q2 {
            Ok(SelectedQuadrant::TopRight)
        } else if *min == std_dev_q3 {
            Ok(SelectedQuadrant::BottomLeft)
        } else {
            Ok(SelectedQuadrant::BottomRight)
        }
    }

    fn margin(&self) -> Margin {
        Margin::unified(self.radius).expect("TODO")
    }
}

fn calculate_std_dev<S>(source: &S, top_left: Point, size: Size) -> f32
where
    S: Lens<Item = HsvPixel>,
{
    let sum: f32 = (0..size.width() as isize)
        .cartesian_product(0..size.height() as isize)
        .map(|(x, y)| {
            source.look(top_left.translate(Offset::new(x, y)).expect("TODO")).expect("TODO").value()
        })
        .sum();

    let mean = sum / size.area() as f32;

    let variance_numerator: f32 = (0..size.width() as isize)
        .cartesian_product(0..size.height() as isize)
        .map(|(x, y)| {
            source.look(top_left.translate(Offset::new(x, y)).expect("TODO")).expect("TODO").value()
        })
        .map(|v| (v - mean).powi(2))
        .sum();

    let variance = variance_numerator / (size.area() - 1) as f32;

    variance.sqrt()
}

struct MeanCalculationInput {
    pixel: Pixel,
    selected_quadrant: SelectedQuadrant,
}

struct MeanCalculationKernel {
    radius: usize,
}

impl Kernel<MeanCalculationInput, Pixel> for MeanCalculationKernel {
    fn apply<S>(&self, source: &S, point: Point) -> IndexResult<Pixel>
    where
        S: Lens<Item = MeanCalculationInput>,
    {
        let input = source.look(point)?;

        let quadrant_size = Size::new(self.radius + 1, self.radius + 1).expect("TODO");

        let result = match input.selected_quadrant {
            SelectedQuadrant::TopLeft => calculate_mean(
                source,
                Point::new(point.x() - self.radius, point.y() - self.radius),
                quadrant_size,
            ),
            SelectedQuadrant::TopRight => calculate_mean(
                source,
                Point::new(point.x(), point.y() - self.radius),
                quadrant_size,
            ),
            SelectedQuadrant::BottomLeft => calculate_mean(
                source,
                Point::new(point.x() - self.radius, point.y()),
                quadrant_size,
            ),
            SelectedQuadrant::BottomRight => {
                calculate_mean(source, Point::new(point.x(), point.y()), quadrant_size)
            }
        };

        Ok(Pixel::new([result.red as u8, result.green as u8, result.blue as u8, input.pixel.a()]))
    }

    fn margin(&self) -> Margin {
        Margin::unified(self.radius).expect("TODO")
    }
}

struct IntermediatePixel {
    red: u16,
    green: u16,
    blue: u16,
}

impl Add for IntermediatePixel {
    type Output = IntermediatePixel;

    fn add(self, rhs: Self) -> Self::Output {
        IntermediatePixel {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Div<u16> for IntermediatePixel {
    type Output = IntermediatePixel;

    fn div(self, rhs: u16) -> Self::Output {
        IntermediatePixel { red: self.red / rhs, green: self.green / rhs, blue: self.blue / rhs }
    }
}

fn calculate_mean<S>(source: &S, top_left: Point, size: Size) -> IntermediatePixel
where
    S: Lens<Item = MeanCalculationInput>,
{
    let sum: IntermediatePixel = (0..size.width() as isize)
        .cartesian_product(0..size.height() as isize)
        .map(|(x, y)| {
            source.look(top_left.translate(Offset::new(x, y)).expect("TODO")).expect("TODO").pixel
        })
        .map(|p| IntermediatePixel { red: p.r() as u16, green: p.g() as u16, blue: p.b() as u16 })
        .reduce(|l, r| l + r)
        .expect("TODO");

    sum / size.area() as u16
}
