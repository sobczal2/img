use crate::{
    component::primitive::{
        Margin,
        Point,
        SizeCreationError,
    },
    lens::{
        overlay::{
            OverlayLensCreationError, OverlayLensCreationResult, OverlayLens
        }, value::ValueLens, Lens
    },
};

/// Create a new [`Lens`] from `source` with a virutal border with the size of
/// `margin` and value of `Value`.
pub fn value_border<S, T>(
    source: S,
    margin: Margin,
    value: T,
) -> OverlayLensCreationResult<impl Lens<Item = T>>
where
    S: Lens<Item = T>,
    T: Clone,
{
    let overlay_size = source.size().extend_by_margin(margin).map_err(|e| match e {
        SizeCreationError::WidthTooBig => OverlayLensCreationError::OverlayTooBig,
        SizeCreationError::HeightTooBig => OverlayLensCreationError::OverlayTooBig,
        _ => unreachable!("unexpected error returned from extend_by_margin"),
    })?;
    OverlayLens::new(
        ValueLens::new(value, overlay_size),
        source,
        // SAFETY: left and top values are less than DIMENSION_MAX
        Point::new(margin.left(), margin.top()).expect("unexpected error in Point::new"),
    )
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::{component::primitive::DIMENSION_MAX, error::IndexError, lens::overlay::OverlayLensCreationError, prelude::Size};

    use super::*;

    #[test]
    fn test_value_border_ok() {
        let no_margin = value_border(ValueLens::new(1, Size::new(1, 1).unwrap()), Margin::new(0, 0, 0, 0).unwrap(), 0);
        assert!(no_margin.is_ok());
        assert_eq!(no_margin.unwrap().size(), Size::new(1, 1).unwrap());

        let with_margin = value_border(ValueLens::new(1, Size::new(1, 1).unwrap()), Margin::new(1, 2, 3, 4).unwrap(), 0);
        assert!(with_margin.is_ok());
        assert_eq!(with_margin.unwrap().size(), Size::new(7, 5).unwrap());
    }

    #[test]
    fn test_value_border_err() {
        assert!(value_border(ValueLens::new(1, Size::new(DIMENSION_MAX, 1).unwrap()), Margin::new(0, 1, 0, 0).unwrap(), 0).is_err_and(|e| e == OverlayLensCreationError::OverlayTooBig));
        assert!(value_border(ValueLens::new(1, Size::new(1, DIMENSION_MAX).unwrap()), Margin::new(1, 0, 0, 0).unwrap(), 0).is_err_and(|e| e == OverlayLensCreationError::OverlayTooBig));
        assert!(value_border(ValueLens::new(1, Size::new(DIMENSION_MAX, 1).unwrap()), Margin::new(0, 0, 0, 1).unwrap(), 0).is_err_and(|e| e == OverlayLensCreationError::OverlayTooBig));
        assert!(value_border(ValueLens::new(1, Size::new(1, DIMENSION_MAX).unwrap()), Margin::new(0, 0, 1, 0).unwrap(), 0).is_err_and(|e| e == OverlayLensCreationError::OverlayTooBig));
    }

    #[test]
    fn test_value_border_works() {
        let lens = value_border(ValueLens::new(1, Size::new(1, 1).unwrap()), Margin::new(1, 2, 3, 4).unwrap(), 0).unwrap();
        let expected_one_point = Point::new(4, 1).unwrap();

        for point in (0..7).cartesian_product(0..5).map(|(x, y)| Point::new(x, y).unwrap()) {
            if point == expected_one_point {
                assert_eq!(lens.look(point).unwrap(), 1);
            } else {
                assert_eq!(lens.look(point).unwrap(), 0);
            }
        }

        assert_eq!(lens.size(), Size::new(7, 5).unwrap());
        assert_eq!(lens.look(Point::new(7, 0).unwrap()).unwrap_err(), IndexError::OutOfBounds);
        assert_eq!(lens.look(Point::new(0, 5).unwrap()).unwrap_err(), IndexError::OutOfBounds);
    }
}
