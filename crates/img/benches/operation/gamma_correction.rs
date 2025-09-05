use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::{
    operation::color::gamma_correction::{
        gamma_correction,
        gamma_correction_par,
    },
    pixel::PixelFlags,
};

mod common;

operation_bench!(gamma_correction [0.4, PixelFlags::RGB]);

criterion_group!(benches, gamma_correction_benchmark);
criterion_main!(benches);
