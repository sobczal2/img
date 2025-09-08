use crate::{
    component::primitive::{
        Margin,
        Point,
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
    let overlay_size = source.size() + margin;
    OverlayLens::new(
        ValueLens::new(value, overlay_size),
        source,
        Point::new(margin.left(), margin.top()),
    )
}
