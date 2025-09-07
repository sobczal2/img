use std::marker::PhantomData;

use crate::{error::IndexResult, lens::Lens, primitive::{point::Point, size::Size}};

pub struct QuickSplitLens<S, LF, RF, L, R> {
    source: S,
    left_factory: LF,
    right_factory: RF,
    _phantom_data_l: PhantomData<L>,
    _phantom_data_r: PhantomData<R>,
}

impl<S, LF, RF, L, R> QuickSplitLens<S, LF, RF, L, R> {
    pub fn new(
        source: S,
        left_factory: LF,
        right_factory: RF,
    ) -> Self {
        Self { source, left_factory, right_factory, _phantom_data_l: Default::default(), _phantom_data_r: Default::default() }
    }
}

impl<S, LF, RF, L, R, LL, RL> Lens for QuickSplitLens<S, LF, RF, L, R>
    where
        S: Lens + Clone,
        LF: Fn(S) -> LL,
        RF: Fn(S) -> RL,
        LL: Lens<Item = L>,
        RL: Lens<Item = R>,
{
    type Item = (L, R);

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        let left = (self.left_factory)(self.source.clone());
        let right = (self.right_factory)(self.source.clone());
        Ok((left.look(point)?, right.look(point)?))
    }

    fn size(&self) -> Size {
        let left = (self.left_factory)(self.source.clone()).size();
        let right = (self.right_factory)(self.source.clone()).size();

        Size::from_usize(
            left.width().min(right.width()),
            left.height().min(right.height())
        ).unwrap()
    }
}
