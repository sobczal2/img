use thiserror::Error;

use crate::{
    error::IndexResult,
    lens::Lens,
    primitive::{
        area::Area,
        offset::Offset,
        point::Point,
        size::Size,
    },
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("overlay_start out of bounds")]
    OverlayStartOutOfBounds,
    #[error("overlay is too big")]
    OverlayTooBig,
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

pub struct OverlayLens<S1, S2> {
    base: S1,
    overlay: S2,
    overlay_area: Area,
}

impl<S1, S2> OverlayLens<S1, S2>
where
    S1: Lens,
    S2: Lens,
{
    pub fn new(base: S1, overlay: S2, overlay_start: Point) -> CreationResult<Self> {
        if !base.size().contains(&overlay_start) {
            return Err(CreationError::OverlayStartOutOfBounds);
        }

        let overlay_size = overlay.size();

        let bottom_right = Point::new(
            overlay_size.width() + overlay_start.x(),
            overlay_size.height() + overlay_start.y(),
        );
        if !base.size().contains(&bottom_right) {
            return Err(CreationError::OverlayTooBig);
        }

        Ok(Self { base, overlay, overlay_area: Area::new(overlay_size, overlay_start) })
    }
}

impl<S1, S2, T> Lens for OverlayLens<S1, S2>
where
    S1: Lens<Item = T>,
    S2: Lens<Item = T>,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        if self.overlay_area.contains(&point) {
            let offset = Offset::from(self.overlay_area.top_left());
            return self.overlay.look(point.translate(-offset).unwrap());
        }

        self.base.look(point)
    }

    fn size(&self) -> Size {
        self.base.size()
    }
}
