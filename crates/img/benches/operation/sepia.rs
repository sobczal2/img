use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::{
    operation::color::sepia::{
        sepia,
        sepia_par,
    },
    pixel::PixelFlags,
};

mod common;

operation_bench!(sepia[PixelFlags::RGB]);

criterion_group!(benches, sepia_benchmark);
criterion_main!(benches);
