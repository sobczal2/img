use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(canny []);

criterion_group!(benches, canny_benchmark);
criterion_main!(benches);
