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
) -> overlay::OverlayCreationResult<impl Lens<Item = T>>
where
    S: Lens<Item = T>,
    T: Clone,
{
    let overlay_size = source.size().extend_by_margin(margin).map_err(|e| match e {
        SizeCreationError::WidthTooBig => overlay::OverlayCreationError::OverlayTooBig,
        SizeCreationError::HeightTooBig => overlay::OverlayCreationError::OverlayTooBig,
        _ => unreachable!("unexpected error returned from extend_by_margin"),
    })?;
    OverlayLens::new(
        ValueLens::new(value, overlay_size),
        source,
        // SAFETY: left and top values are less than DIMENSION_MAX
        Point::new(margin.left(), margin.top()).expect("unexpected error in Point::new"),
    )
}
