use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::{
    operation::geometry::crop::{
        crop,
        crop_par,
    },
    primitive::margin::Margin,
};

mod common;

operation_bench!(crop[Margin::unified(3)]);

criterion_group!(benches, crop_benchmark);
criterion_main!(benches);
