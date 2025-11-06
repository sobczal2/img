use crate::{
    component::primitive::{
        Margin,
        Point,
        SizeCreationError,
    },
    lens::{
        Lens,
        overlay::{
            self,
            OverlayLens,
        },
        value::ValueLens,
    },
};

pub fn value_border<S, T>(
    source: S,
    margin: Margin,
    value: T,
) -> overlay::CreationResult<impl Lens<Item = T>>
where
    S: Lens<Item = T>,
    T: Clone,
{
    let overlay_size = source.size().extend_by_margin(margin).map_err(|e| match e {
        SizeCreationError::WidthTooBig => overlay::CreationError::OverlayTooBig,
        SizeCreationError::HeightTooBig => overlay::CreationError::OverlayTooBig,
        _ => unreachable!("unexpected error returned from extend_by_margin"),
    })?;
    OverlayLens::new(
        ValueLens::new(value, overlay_size),
        source,
        Point::new(margin.left(), margin.top()),
    )
}
