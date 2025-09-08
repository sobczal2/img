use std::fmt;

use img::component::kernel::identity::IdentityKernel;
use img::lens::value::ValueLens;
use img::lens::Lens;
use img::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use paste::paste;

fn assert_lens_size_sync<S, R>(lens: &S, rng: &mut R)
where S: Lens, R: Rng
{
    let x: u32 = rng.random::<u32>() % 100;
    let y: u32 = rng.random::<u32>() % 100;
    let point = Point::new(x as usize, y as usize);
    assert_eq!(lens.look(point).is_ok(), lens.size().contains(&point));
}

fn assert_lens_repeated_look<S, R>(lens: &S, rng: &mut R)
where S: Lens, R: Rng, S::Item: fmt::Debug + Eq,
{
    let x: u32 = rng.random::<u32>() % 100;
    let y: u32 = rng.random::<u32>() % 100;
    let point = Point::new(x as usize, y as usize);
    assert_eq!(lens.look(point), lens.look(point));
}

fn prepare_test_image(width: usize, height: usize) -> &'static Image {
    let mut rng = SmallRng::seed_from_u64(0);
    
    Box::leak(Box::new(Image::random(Size::from_usize(width, height).unwrap(), &mut rng)))
}

macro_rules! test_lens {
    ($name:ident, $lens:expr, $iterations:expr) => {
        paste! {
            #[test]
            pub fn [<test_ $name>]() {
                let mut rng = SmallRng::seed_from_u64(0);
                let lens = $lens;
                for _ in 0..$iterations {
                    assert_lens_size_sync(&lens, &mut rng);
                    assert_lens_repeated_look(&lens, &mut rng);
                }
            }
        }
    };
}

fn prepare_image_lens(width: usize, height: usize) -> impl Lens<Item = &'static Pixel> {
    prepare_test_image(width, height).lens()
}

test_lens!(image_lens, prepare_image_lens(50, 100), 100);

fn prepare_map_lens(width: usize, height: usize) -> impl Lens<Item = u8> {
    prepare_test_image(width, height).lens().map(|s| s.r())
}

test_lens!(map_lens, prepare_map_lens(50, 100), 100);


fn prepare_split2_lens(width: usize, height: usize) -> impl Lens<Item = (&'static Pixel, &'static Pixel)> {
    prepare_test_image(width, height).lens().split2(|s| s, |s| s)
}

test_lens!(split2_lens, prepare_split2_lens(50, 100), 100);

fn prepare_split3_lens(width: usize, height: usize) -> impl Lens<Item = (&'static Pixel, &'static Pixel, &'static Pixel)> {
    prepare_test_image(width, height).lens().split3(|s| s, |s| s, |s| s)
}

test_lens!(split3_lens, prepare_split3_lens(50, 100), 100);

fn prepare_split4_lens(width: usize, height: usize) -> impl Lens<Item = (&'static Pixel, &'static Pixel, &'static Pixel, &'static Pixel)> {
    prepare_test_image(width, height).lens().split4(|s| s, |s| s, |s| s, |s| s)
}

test_lens!(split4_lens, prepare_split4_lens(50, 100), 100);

fn prepare_value_lens(width: usize, height: usize) -> impl Lens<Item = u8> {
    ValueLens::new(0u8, Size::from_usize(width, height).unwrap())
}

test_lens!(value_lens, prepare_value_lens(50, 100), 100);

fn prepare_remap_lens(width: usize, height: usize) -> impl Lens<Item = &'static Pixel> {
    prepare_test_image(width, height).lens().remap(|s, point| s.look(point), Size::from_usize(width, height).unwrap())
}

test_lens!(remap_lens, prepare_remap_lens(50, 100), 100);

fn prepare_cloned_lens(width: usize, height: usize) -> impl Lens<Item = Pixel> {
    prepare_test_image(width, height).lens().cloned()
}

test_lens!(cloned_lens, prepare_cloned_lens(50, 100), 100);

fn prepare_materialize_lens(width: usize, height: usize) -> impl Lens<Item = &'static Pixel> {
    prepare_test_image(width, height).lens().materialize()
}

test_lens!(materialize_lens, prepare_materialize_lens(50, 100), 100);

fn prepare_kernel_lens(width: usize, height: usize) -> impl Lens<Item = &'static Pixel> {
    prepare_test_image(width, height).lens().kernel(IdentityKernel::new()).unwrap()
}

test_lens!(kernel_lens, prepare_kernel_lens(50, 100), 100);

fn prepare_overlay_lens(width: usize, height: usize) -> impl Lens<Item = Pixel> {
    prepare_test_image(width, height).lens().cloned().overlay(ValueLens::new(Pixel::zero(), Size::from_usize(20, 30).unwrap()), Point::new(20, 10)).unwrap()
}

test_lens!(overlay_lens, prepare_overlay_lens(50, 100), 100);

