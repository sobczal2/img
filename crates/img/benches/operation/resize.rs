use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(resize[Scale::new(0.5, 0.5).unwrap()]);

criterion_group!(benches, resize_benchmark);
criterion_main!(benches);
