use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::IndexResult,
    image::Image,
    lens::Lens,
    pixel::Pixel,
};

#[derive(Clone)]
pub struct ImageLens<'a>(&'a Image);

impl<'a> ImageLens<'a> {
    pub fn new(image: &'a Image) -> Self {
        Self(image)
    }
}

impl<'a> Lens for ImageLens<'a> {
    type Item = &'a Pixel;

    fn look(&self, point: Point) -> IndexResult<&'a Pixel> {
        self.0.pixel(point)
    }

    fn size(&self) -> Size {
        self.0.size()
    }
}
