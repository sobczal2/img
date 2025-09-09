use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(mean_blur [2, PixelFlags::RGB]);

criterion_group!(benches, mean_blur_benchmark);
criterion_main!(benches);
