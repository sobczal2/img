use std::cell::RefCell;

use crate::{
    component::{
        kernel::{
            Kernel,
            kernel_margin_for_size,
        },
        primitive::histogram::Histogram,
    },
    error::{
        IndexError,
        IndexResult,
    },
    lens::Lens,
    pixel::{
        ChannelFlags,
        ChannelStore,
        Pixel,
    },
    prelude::{
        Area,
        Offset,
        Point,
        Size,
    },
};

struct PreviousResult {
    point: Point,
    histograms: ChannelStore<Histogram<u16, 256>>,
}

pub struct MedianKernel {
    previous_result: RefCell<Option<PreviousResult>>,
    size: Size,
    flags: ChannelFlags,
}

impl MedianKernel {
    pub fn new(size: Size, flags: ChannelFlags) -> MedianKernel {
        Self { previous_result: Default::default(), size, flags }
    }
}

impl<In> Kernel<In, Pixel> for MedianKernel
where
    In: AsRef<Pixel>,
{
    fn apply<S>(&self, source: &S, point: Point) -> IndexResult<Pixel>
    where
        S: Lens<Item = In>,
    {
        let working_area = Area::from_cropped_size(
            source.size(),
            <MedianKernel as Kernel<In, Pixel>>::margin(self),
        )
        .expect("failed to create working area, this is either lens or kernel bug");

        if !working_area.contains(&point) {
            return Err(IndexError::OutOfBounds);
        }

        let center_offset: Offset = self.size.middle().into();
        let kernel_area = Area::new(
            self.size,
            point.translate(-center_offset).expect("unexpected error in Point::translate"),
        );

        let previous_result_present = { self.previous_result.borrow().is_some() };

        if previous_result_present {
            Ok(self.previous_result_impl(source, point, kernel_area))
        } else {
            Ok(self.no_previous_result_impl(source, point, kernel_area))
        }
    }

    fn margin(&self) -> crate::prelude::Margin {
        kernel_margin_for_size(self.size)
    }
}

impl MedianKernel {
    fn no_previous_result_impl<S, In>(&self, source: &S, point: Point, kernel_area: Area) -> Pixel
    where
        S: Lens<Item = In>,
        In: AsRef<Pixel>,
    {
        let original = source.look(point).expect("unexpected error in Lens::look");
        let original = original.as_ref();

        let mut histograms = ChannelStore::from_fn(Histogram::<u16, 256>::default);

        for point in kernel_area.points() {
            let current = source.look(point).expect("unexpected error in Lens::look");
            let current: ChannelStore<_> = (*current.as_ref()).into();
            histograms
                .zip_active(&current, self.flags, |h, c| h.insert_index((*c).into()))
                .expect("unexpected error in Histogram::insert_index");
        }

        let result_channels = histograms.map_with((*original).into(), |h, orig, flag| {
            if self.flags.contains(flag) {
                h.median()
                    .expect("Median calculation failed")
                    .try_into()
                    .expect("Median calculation failed")
            } else {
                orig
            }
        });

        let pixel = Pixel::new(result_channels.into());

        *self.previous_result.borrow_mut() = Some(PreviousResult { point, histograms });

        pixel
    }

    fn previous_result_impl<S, In>(&self, source: &S, point: Point, kernel_area: Area) -> Pixel
    where
        S: Lens<Item = In>,
        In: AsRef<Pixel>,
    {
        let original = source.look(point).expect("unexpected error in Lens::look");
        let original = original.as_ref();

        let previous_result_valid = {
            let previous_result = self.previous_result.borrow();
            let previous_result =
                previous_result.as_ref().expect("unexpected none in MedianKernel.previous_result");
            point.x() + 1 != previous_result.point.x() || point.y() != previous_result.point.y()
        };

        if previous_result_valid {
            return self.no_previous_result_impl(source, point, kernel_area);
        }

        let mut histograms = self
            .previous_result
            .take()
            .expect("unexpected none in MedianKernel.previous_result")
            .histograms;

        for point in (kernel_area.top_left().y()..=kernel_area.bottom_left().y()).map(|y| {
            Point::new(kernel_area.top_left().x(), y).expect("unexpected error in Point::new")
        }) {
            let current = source.look(point).expect("unexpected error in Lens::look");
            let current: ChannelStore<_> = (*current.as_ref()).into();
            histograms
                .zip_active(&current, self.flags, |h, c| h.remove_index((*c).into()))
                .expect("unexpected error in Histogram::insert_index");
        }

        for point in (kernel_area.top_right().y()..=kernel_area.bottom_right().y()).map(|y| {
            Point::new(kernel_area.top_left().x(), y).expect("unexpected error in Point::new")
        }) {
            let current = source.look(point).expect("unexpected error in Lens::look");
            let current: ChannelStore<_> = (*current.as_ref()).into();
            histograms
                .zip_active(&current, self.flags, |h, c| h.insert_index((*c).into()))
                .expect("unexpected error in Histogram::insert_index");
        }

        let result_channels = histograms.map_with((*original).into(), |h, orig, flag| {
            if self.flags.contains(flag) {
                h.median()
                    .expect("Median calculation failed")
                    .try_into()
                    .expect("Median calculation failed")
            } else {
                orig
            }
        });

        let pixel = Pixel::new(result_channels.into());

        *self.previous_result.borrow_mut() = Some(PreviousResult { point, histograms });

        pixel
    }
}
