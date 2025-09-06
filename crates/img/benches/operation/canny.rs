use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::operation::detection::edge::canny::{
    canny,
    canny_par,
};

mod common;

operation_bench!(canny []);

criterion_group!(benches, canny_benchmark);
criterion_main!(benches);
