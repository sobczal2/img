use crate::{
    error::IndexResult,
    image::Image,
    lens::{
        Lens,
        image::colors::{
            AlphaColorLens,
            BlueColorLens,
            ColorsLens,
            GreenColorLens,
            RedColorLens,
        },
    },
    pixel::Pixel,
    primitive::{
        point::Point,
        size::Size,
    },
};

pub mod colors;

#[derive(Clone)]
pub struct ImageLens<'a>(&'a Image);

impl<'a> ImageLens<'a> {
    pub fn new(image: &'a Image) -> Self {
        Self(image)
    }
}

impl<'a> Lens for ImageLens<'a> {
    type Item = &'a Pixel;

    fn get(&self, point: Point) -> IndexResult<&'a Pixel> {
        self.0.pixel(point)
    }

    fn size(&self) -> Size {
        self.0.size()
    }
}

pub trait PixelLens<T>: Lens<Item = T>
where
    T: AsRef<Pixel>,
{
    fn colors<R, G, B, A, RF, GF, BF, AF, E>(
        self,
        red: RF,
        green: GF,
        blue: BF,
        alpha: AF,
    ) -> Result<ColorsLens<R, G, B, A>, E>
    where
        Self: Sized,
        RF: FnOnce(RedColorLens) -> Result<R, E>,
        GF: FnOnce(GreenColorLens) -> Result<G, E>,
        BF: FnOnce(BlueColorLens) -> Result<B, E>,
        AF: FnOnce(AlphaColorLens) -> Result<A, E>,
        R: Lens<Item = u8>,
        G: Lens<Item = u8>,
        B: Lens<Item = u8>,
        A: Lens<Item = u8>,
    {
        ColorsLens::new(self, red, green, blue, alpha)
    }
}

impl<P, T> PixelLens<T> for P
where
    P: Lens<Item = T>,
    T: AsRef<Pixel>,
{
}
