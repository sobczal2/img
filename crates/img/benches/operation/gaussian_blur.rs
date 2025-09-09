use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(gaussian_blur [2, 3.0, ChannelFlags::RGB]);

criterion_group!(benches, gaussian_blur_benchmark);
criterion_main!(benches);
