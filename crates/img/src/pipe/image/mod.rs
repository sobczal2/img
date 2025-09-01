use crate::{
    error::IndexResult,
    image::Image,
    pipe::{
        Pipe,
        image::colors::{
            AlphaColorPipe,
            BlueColorPipe,
            ColorsPipe,
            GreenColorPipe,
            RedColorPipe,
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
pub struct ImagePipe<'a>(&'a Image);

impl<'a> ImagePipe<'a> {
    pub fn new(image: &'a Image) -> Self {
        Self(image)
    }
}

impl<'a> Pipe for ImagePipe<'a> {
    type Item = &'a Pixel;

    fn get(&self, point: Point) -> IndexResult<&'a Pixel> {
        self.0.pixel(point)
    }

    fn size(&self) -> Size {
        self.0.size()
    }
}

pub trait PixelPipe<T>: Pipe<Item = T>
where
    T: AsRef<Pixel>,
{
    fn colors<R, G, B, A, RF, GF, BF, AF, E>(
        self,
        red: RF,
        green: GF,
        blue: BF,
        alpha: AF,
    ) -> Result<ColorsPipe<R, G, B, A>, E>
    where
        Self: Sized,
        RF: FnOnce(RedColorPipe) -> Result<R, E>,
        GF: FnOnce(GreenColorPipe) -> Result<G, E>,
        BF: FnOnce(BlueColorPipe) -> Result<B, E>,
        AF: FnOnce(AlphaColorPipe) -> Result<A, E>,
        R: Pipe<Item = u8>,
        G: Pipe<Item = u8>,
        B: Pipe<Item = u8>,
        A: Pipe<Item = u8>,
    {
        ColorsPipe::new(self, red, green, blue, alpha)
    }
}

impl<P, T> PixelPipe<T> for P
where
    P: Pipe<Item = T>,
    T: AsRef<Pixel>,
{
}
