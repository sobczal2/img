use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(gamma_correction [0.4, ChannelFlags::RGB]);

criterion_group!(benches, gamma_correction_benchmark);
criterion_main!(benches);
