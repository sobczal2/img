use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(grayscale[PixelFlags::RGB]);

criterion_group!(benches, grayscale_benchmark);
criterion_main!(benches);
