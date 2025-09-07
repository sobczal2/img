use crate::{
    error::IndexResult,
    lens::{
        Lens,
        materialize::MaterializeLens,
    },
    primitive::{
        point::Point,
        size::Size,
    },
};

pub struct SplitLens<L, R> {
    left: L,
    right: R,
    size: Size,
}

impl<L, R> SplitLens<L, R> {
    pub fn new<S, LF, RF, E>(source: S, left_factory: LF, right_factory: RF) -> Result<Self, E>
    where
        S: Lens,
        LF: FnOnce(MaterializeLens<S::Item>) -> Result<L, E>,
        RF: FnOnce(MaterializeLens<S::Item>) -> Result<R, E>,
        L: Lens,
        R: Lens,
    {
        let materialize = MaterializeLens::new(source);

        let left = (left_factory)(materialize.clone())?;
        let right = (right_factory)(materialize.clone())?;

        let min_width = left.size().width().min(right.size().width());
        let min_height = left.size().height().min(right.size().height());

        let size = Size::from_usize(min_width, min_height).unwrap();

        Ok(Self { left, right, size })
    }

    pub fn new_par<S, LF, RF, E>(source: S, left_factory: LF, right_factory: RF) -> Result<Self, E>
    where
        S: Lens + Send + Sync,
        S::Item: Send,
        LF: FnOnce(MaterializeLens<S::Item>) -> Result<L, E>,
        RF: FnOnce(MaterializeLens<S::Item>) -> Result<R, E>,
        L: Lens,
        R: Lens,
    {
        let materialize = MaterializeLens::new_par(source);

        let left = (left_factory)(materialize.clone())?;
        let right = (right_factory)(materialize.clone())?;

        let min_width = left.size().width().min(right.size().width());
        let min_height = left.size().height().min(right.size().height());

        let size = Size::from_usize(min_width, min_height).unwrap();

        Ok(Self { left, right, size })
    }
}

impl<L, R> Lens for SplitLens<L, R>
where
    L: Lens,
    R: Lens,
{
    type Item = (L::Item, R::Item);

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        Ok((self.left.look(point)?, self.right.look(point)?))
    }

    fn size(&self) -> Size {
        self.size
    }
}
