use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(kuwahara[]);

criterion_group!(benches, kuwahara_benchmark);
criterion_main!(benches);
