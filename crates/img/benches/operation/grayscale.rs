use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::{
    operation::color::grayscale::{
        grayscale,
        grayscale_par,
    },
    pixel::PixelFlags,
};

mod common;

operation_bench!(grayscale[PixelFlags::RGB]);

criterion_group!(benches, grayscale_benchmark);
criterion_main!(benches);
