use crate::{
    component::primitive::{
        Point,
        Size,
    },
    error::IndexResult,
    lens::Lens,
};

pub struct SplitLens2<L1, L2> {
    lens1: L1,
    lens2: L2,
    size: Size,
}

impl<L1, L2, D1, D2> SplitLens2<L1, L2>
where
    L1: Lens<Item = D1>,
    L2: Lens<Item = D2>,
{
    pub fn new<S, F1, F2>(source: S, factory1: F1, factory2: F2) -> Self
    where
        S: Lens + Clone,
        F1: Fn(S) -> L1,
        F2: Fn(S) -> L2,
    {
        let lens1 = (factory1)(source.clone());
        let lens2 = (factory2)(source.clone());

        // SAFETY: taking minimum of `Lens` `Size`'s each dimension produces
        // a valid `Size`.
        let size = Size::new(
            lens1.size().width().min(lens2.size().width()),
            lens1.size().height().min(lens2.size().height()),
        )
        .unwrap();

        Self { lens1, lens2, size }
    }
}

impl<L1, L2, D1, D2> Lens for SplitLens2<L1, L2>
where
    L1: Lens<Item = D1>,
    L2: Lens<Item = D2>,
{
    type Item = (D1, D2);

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        Ok((self.lens1.look(point)?, self.lens2.look(point)?))
    }

    fn size(&self) -> Size {
        self.size
    }
}

pub struct SplitLens3<L1, L2, L3> {
    lens1: L1,
    lens2: L2,
    lens3: L3,
    size: Size,
}

impl<L1, L2, L3, D1, D2, D3> SplitLens3<L1, L2, L3>
where
    L1: Lens<Item = D1>,
    L2: Lens<Item = D2>,
    L3: Lens<Item = D3>,
{
    pub fn new<S, F1, F2, F3>(source: S, factory1: F1, factory2: F2, factory3: F3) -> Self
    where
        S: Lens + Clone,
        F1: Fn(S) -> L1,
        F2: Fn(S) -> L2,
        F3: Fn(S) -> L3,
    {
        let lens1 = (factory1)(source.clone());
        let lens2 = (factory2)(source.clone());
        let lens3 = (factory3)(source.clone());

        // SAFETY: taking minimum of `Lens` `Size`'s each dimension produces
        // a valid `Size`.
        let size = Size::new(
            lens1.size().width().min(lens2.size().width()).min(lens3.size().width()),
            lens1.size().height().min(lens2.size().height()).min(lens3.size().height()),
        )
        .unwrap();

        Self { lens1, lens2, lens3, size }
    }
}

impl<L1, L2, L3, D1, D2, D3> Lens for SplitLens3<L1, L2, L3>
where
    L1: Lens<Item = D1>,
    L2: Lens<Item = D2>,
    L3: Lens<Item = D3>,
{
    type Item = (D1, D2, D3);

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        Ok((self.lens1.look(point)?, self.lens2.look(point)?, self.lens3.look(point)?))
    }

    fn size(&self) -> Size {
        self.size
    }
}

pub struct SplitLens4<L1, L2, L3, L4> {
    lens1: L1,
    lens2: L2,
    lens3: L3,
    lens4: L4,
    size: Size,
}

impl<L1, L2, L3, L4, D1, D2, D3, D4> SplitLens4<L1, L2, L3, L4>
where
    L1: Lens<Item = D1>,
    L2: Lens<Item = D2>,
    L3: Lens<Item = D3>,
    L4: Lens<Item = D4>,
{
    pub fn new<S, F1, F2, F3, F4>(
        source: S,
        factory1: F1,
        factory2: F2,
        factory3: F3,
        factory4: F4,
    ) -> Self
    where
        S: Lens + Clone,
        F1: Fn(S) -> L1,
        F2: Fn(S) -> L2,
        F3: Fn(S) -> L3,
        F4: Fn(S) -> L4,
    {
        let lens1 = (factory1)(source.clone());
        let lens2 = (factory2)(source.clone());
        let lens3 = (factory3)(source.clone());
        let lens4 = (factory4)(source.clone());

        // SAFETY: taking minimum of `Lens` `Size`'s each dimension produces
        // a valid `Size`.
        let size = Size::new(
            lens1
                .size()
                .width()
                .min(lens2.size().width())
                .min(lens3.size().width())
                .min(lens4.size().width()),
            lens1
                .size()
                .height()
                .min(lens2.size().height())
                .min(lens3.size().height())
                .min(lens4.size().height()),
        )
        .unwrap();

        Self { lens1, lens2, lens3, lens4, size }
    }
}

impl<L1, L2, L3, L4, D1, D2, D3, D4> Lens for SplitLens4<L1, L2, L3, L4>
where
    L1: Lens<Item = D1>,
    L2: Lens<Item = D2>,
    L3: Lens<Item = D3>,
    L4: Lens<Item = D4>,
{
    type Item = (D1, D2, D3, D4);

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        Ok((
            self.lens1.look(point)?,
            self.lens2.look(point)?,
            self.lens3.look(point)?,
            self.lens4.look(point)?,
        ))
    }

    fn size(&self) -> Size {
        self.size
    }
}
