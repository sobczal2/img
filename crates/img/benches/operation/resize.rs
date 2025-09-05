use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::{
    operation::geometry::resize::{
        resize,
        resize_par,
    },
    primitive::scale::Scale,
};

mod common;

operation_bench!(resize[Scale::new(0.5, 0.5).unwrap()]);

criterion_group!(benches, resize_benchmark);
criterion_main!(benches);
