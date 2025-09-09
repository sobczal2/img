use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(crop[Margin::unified(3)]);

criterion_group!(benches, crop_benchmark);
criterion_main!(benches);
