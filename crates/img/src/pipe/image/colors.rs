use crate::{
    error::{
        IndexResult,
        OutOfBoundsError,
    },
    pipe::{
        Pipe,
        image::PixelPipe,
        materialize::MaterializePipe,
    },
    pixel::Pixel,
    primitive::{
        point::Point,
        size::Size,
    },
};

pub struct ColorsPipe<R, G, B, A> {
    red: R,
    green: G,
    blue: B,
    alpha: A,
    size: Size,
}

macro_rules! color_pipe {
    ($name:ident, $method:ident) => {
        pub struct $name {
            source: MaterializePipe<Pixel>,
        }

        impl Pipe for $name {
            type Item = u8;

            fn get(&self, point: Point) -> IndexResult<Self::Item> {
                Ok(self.source.get(point)?.$method())
            }

            fn size(&self) -> Size {
                self.source.size()
            }
        }
    };
}

color_pipe!(RedColorPipe, r);
color_pipe!(GreenColorPipe, g);
color_pipe!(BlueColorPipe, b);
color_pipe!(AlphaColorPipe, a);

impl<R, G, B, A> ColorsPipe<R, G, B, A> {
    pub fn new<S, RF, GF, BF, AF, T, E>(
        source: S,
        red: RF,
        green: GF,
        blue: BF,
        alpha: AF,
    ) -> Result<Self, E>
    where
        S: PixelPipe<T>,
        T: AsRef<Pixel>,
        RF: FnOnce(RedColorPipe) -> Result<R, E>,
        GF: FnOnce(GreenColorPipe) -> Result<G, E>,
        BF: FnOnce(BlueColorPipe) -> Result<B, E>,
        AF: FnOnce(AlphaColorPipe) -> Result<A, E>,
        R: Pipe<Item = u8>,
        G: Pipe<Item = u8>,
        B: Pipe<Item = u8>,
        A: Pipe<Item = u8>,
    {
        let materialize = MaterializePipe::new(source.map(|p| *p.as_ref()));

        let red = (red)(RedColorPipe { source: materialize.clone() })?;
        let green = (green)(GreenColorPipe { source: materialize.clone() })?;
        let blue = (blue)(BlueColorPipe { source: materialize.clone() })?;
        let alpha = (alpha)(AlphaColorPipe { source: materialize.clone() })?;

        let sizes = [red.size(), green.size(), blue.size(), alpha.size()];

        let min_width = sizes.iter().map(|s| s.width()).min().unwrap();
        let min_height = sizes.iter().map(|s| s.height()).min().unwrap();

        let size = Size::from_usize(min_width, min_height).unwrap();

        Ok(Self { red, green, blue, alpha, size })
    }
}

impl<R, G, B, A> Pipe for ColorsPipe<R, G, B, A>
where
    R: Pipe<Item = u8>,
    G: Pipe<Item = u8>,
    B: Pipe<Item = u8>,
    A: Pipe<Item = u8>,
{
    type Item = Pixel;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        if !self.size.contains(point) {
            return Err(OutOfBoundsError);
        }

        let red = self.red.get(point).expect("bug in red pipe implementation");
        let green = self.green.get(point).expect("bug in green pipe implementation");
        let blue = self.blue.get(point).expect("bug in blue pipe implementation");
        let alpha = self.alpha.get(point).expect("bug in alpha pipe implementation");

        Ok(Pixel::new([red, green, blue, alpha]))
    }

    fn size(&self) -> Size {
        self.size
    }
}
