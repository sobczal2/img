use crate::{
    error::{
        IndexResult,
        OutOfBoundsError,
    },
    lens::Lens,
    primitive::{
        area::Area,
        margin::Margin,
        offset::Offset,
        point::Point,
        size::Size,
    },
};

// TODO: consider trait instead
#[derive(Clone)]
pub enum BorderFill {
    PickZero,
}

#[derive(Clone)]
pub struct BorderLens<S>
where
    S: Lens,
{
    source: S,
    margin: Margin,
    fill: BorderFill,
}

impl<S> BorderLens<S>
where
    S: Lens,
{
    pub fn new(source: S, margin: Margin, fill: BorderFill) -> Self {
        Self { source, margin, fill }
    }
}

impl<S> Lens for BorderLens<S>
where
    S: Lens,
{
    type Item = S::Item;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        let source_area = Area::from_cropped_size(self.size(), self.margin).unwrap();

        if source_area.contains(&point) {
            let offset: Offset = source_area.top_left().into();
            self.source.look(point.translate(-offset).unwrap())
        } else if self.size().contains(&point) {
            match &self.fill {
                BorderFill::PickZero => self.source.look(Point::zero()),
            }
        } else {
            Err(OutOfBoundsError)
        }
    }

    fn size(&self) -> Size {
        self.source.size() + self.margin
    }
}
