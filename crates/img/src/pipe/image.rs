use crate::{
    error::IndexResult,
    image::Image,
    pipe::Pipe,
    pixel::Pixel,
    primitive::{point::Point, size::Size},
};

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
