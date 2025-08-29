use crate::{error::IndexResult, image::Image, pixel::Pixel, primitives::{point::Point, size::Size}, view::{AsView, ElementsIterator, View}};

pub struct ImageView<'a>(&'a Image);

impl<'a> ImageView<'a> {
    pub fn elements(&self) -> ElementsIterator<Self, Pixel<'a>> {
        ElementsIterator::new(self)
    }
}

impl<'a> View<Pixel<'a>> for ImageView<'a>
{
    fn get(&self, point: Point) -> IndexResult<Pixel<'a>> {
        self.0.pixel(point)
    }

    fn size(&self) -> Size {
        self.0.size()
    }
}

impl<'a> AsView<'a, ImageView<'a>, Pixel<'a>> for Image {
    fn as_view(&'a self) -> ImageView<'a> {
        ImageView(self)
    }
}
