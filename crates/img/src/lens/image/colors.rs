use crate::{
    error::{
        IndexResult,
        OutOfBoundsError,
    },
    lens::{
        Lens,
        image::PixelLens,
        materialize::MaterializeLens,
    },
    pixel::Pixel,
    primitive::{
        point::Point,
        size::Size,
    },
};

pub struct ColorsLens<R, G, B, A> {
    red: R,
    green: G,
    blue: B,
    alpha: A,
    size: Size,
}

macro_rules! color_lens {
    ($name:ident, $method:ident) => {
        pub struct $name {
            source: MaterializeLens<Pixel>,
        }

        impl Lens for $name {
            type Item = u8;

            fn look(&self, point: Point) -> IndexResult<Self::Item> {
                Ok(self.source.look(point)?.$method())
            }

            fn size(&self) -> Size {
                self.source.size()
            }
        }
    };
}

color_lens!(RedColorLens, r);
color_lens!(GreenColorLens, g);
color_lens!(BlueColorLens, b);
color_lens!(AlphaColorLens, a);

impl<R, G, B, A> ColorsLens<R, G, B, A> {
    pub fn new<S, RF, GF, BF, AF, E>(
        source: S,
        red: RF,
        green: GF,
        blue: BF,
        alpha: AF,
    ) -> Result<Self, E>
    where
        S: PixelLens,
        RF: FnOnce(RedColorLens) -> Result<R, E>,
        GF: FnOnce(GreenColorLens) -> Result<G, E>,
        BF: FnOnce(BlueColorLens) -> Result<B, E>,
        AF: FnOnce(AlphaColorLens) -> Result<A, E>,
        R: Lens<Item = u8>,
        G: Lens<Item = u8>,
        B: Lens<Item = u8>,
        A: Lens<Item = u8>,
    {
        let materialize = MaterializeLens::new(source);

        let red = (red)(RedColorLens { source: materialize.clone() })?;
        let green = (green)(GreenColorLens { source: materialize.clone() })?;
        let blue = (blue)(BlueColorLens { source: materialize.clone() })?;
        let alpha = (alpha)(AlphaColorLens { source: materialize.clone() })?;

        let sizes = [red.size(), green.size(), blue.size(), alpha.size()];

        let min_width = sizes.iter().map(|s| s.width()).min().unwrap();
        let min_height = sizes.iter().map(|s| s.height()).min().unwrap();

        let size = Size::from_usize(min_width, min_height).unwrap();

        Ok(Self { red, green, blue, alpha, size })
    }

    #[cfg(feature = "parallel")]
    pub fn new_par<S, RF, GF, BF, AF, E>(
        source: S,
        red: RF,
        green: GF,
        blue: BF,
        alpha: AF,
    ) -> Result<Self, E>
    where
        S: PixelLens + Send + Sync,
        RF: FnOnce(RedColorLens) -> Result<R, E>,
        GF: FnOnce(GreenColorLens) -> Result<G, E>,
        BF: FnOnce(BlueColorLens) -> Result<B, E>,
        AF: FnOnce(AlphaColorLens) -> Result<A, E>,
        R: Lens<Item = u8>,
        G: Lens<Item = u8>,
        B: Lens<Item = u8>,
        A: Lens<Item = u8>,
    {
        let materialize = MaterializeLens::new_par(source);

        let red = (red)(RedColorLens { source: materialize.clone() })?;
        let green = (green)(GreenColorLens { source: materialize.clone() })?;
        let blue = (blue)(BlueColorLens { source: materialize.clone() })?;
        let alpha = (alpha)(AlphaColorLens { source: materialize.clone() })?;

        let sizes = [red.size(), green.size(), blue.size(), alpha.size()];

        let min_width = sizes.iter().map(|s| s.width()).min().unwrap();
        let min_height = sizes.iter().map(|s| s.height()).min().unwrap();

        let size = Size::from_usize(min_width, min_height).unwrap();

        Ok(Self { red, green, blue, alpha, size })
    }
}

impl<R, G, B, A> Lens for ColorsLens<R, G, B, A>
where
    R: Lens<Item = u8>,
    G: Lens<Item = u8>,
    B: Lens<Item = u8>,
    A: Lens<Item = u8>,
{
    type Item = Pixel;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        if !self.size.contains(&point) {
            return Err(OutOfBoundsError);
        }

        let red = self.red.look(point).expect("bug in red lens implementation");
        let green = self.green.look(point).expect("bug in green lens implementation");
        let blue = self.blue.look(point).expect("bug in blue lens implementation");
        let alpha = self.alpha.look(point).expect("bug in alpha lens implementation");

        Ok(Pixel::new([red, green, blue, alpha]))
    }

    fn size(&self) -> Size {
        self.size
    }
}
