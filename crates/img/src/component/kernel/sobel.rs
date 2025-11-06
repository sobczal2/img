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
    error::{
        IndexError,
        IndexResult,
    },
    lens::Lens,
};

const SOBEL_X: [[i16; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];

const SOBEL_Y: [[i16; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

#[derive(Debug, Default, Clone, Copy)]
pub struct Gradient {
    x: i16,
    y: i16,
}

impl Gradient {
    pub fn magnitude(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn direction(&self) -> f32 {
        (self.x as f32).atan2(self.y as f32)
    }
}

#[derive(Default, Copy, Clone)]
pub struct SobelKernel;

impl SobelKernel {
    pub fn new() -> Self {
        SobelKernel
    }
}

impl Kernel<u8, Gradient> for SobelKernel {
    fn apply<P>(&self, lens: &P, point: Point) -> IndexResult<Gradient>
    where
        P: Lens<Item = u8>,
    {
        if !in_bounds(lens.size(), point) {
            return Err(IndexError::OutOfBounds);
        }

        let (g_x, g_y) = SOBEL_X
            .iter()
            .zip(SOBEL_Y)
            .enumerate()
            .flat_map(|(y, (row_x, row_y))| {
                row_x.iter().zip(row_y).enumerate().map(move |(x, (x_value, y_value))| {
                    (Offset::new(x as isize - 1, y as isize - 1), x_value, y_value)
                })
            })
            .map(|(offset, x_value, y_value)| {
                let lens_value = lens
                    // SAFETY: bounds check performed before processing
                    .look(point.translate(offset).expect("unexpected error in translate"))
                    // SAFETY: bounds check performed before processing
                    .expect("bug in lens implementation") as i16;
                (x_value * lens_value, y_value * lens_value)
            })
            .reduce(|l, r| (l.0 + r.0, l.1 + r.1))
            // SAFETY: iterator is never empty so reduce always returns `Some`.
            .expect("unexpected error in reduce");

        Ok(Gradient { x: g_x, y: g_y })
    }

    fn margin(&self) -> Margin {
        Margin::unified(1)
    }
}

fn in_bounds(size: Size, point: Point) -> bool {
    point.x() + 1 < size.width() && point.x() > 0 && point.y() + 1 < size.height() && point.y() > 0
}
