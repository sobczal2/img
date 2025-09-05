use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::{
    operation::blur::gaussian::{
        gaussian_blur,
        gaussian_blur_par,
    },
    pixel::PixelFlags,
};

mod common;

operation_bench!(gaussian_blur [2, 3.0, PixelFlags::RGB]);

criterion_group!(benches, gaussian_blur_benchmark);
criterion_main!(benches);
