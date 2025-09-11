use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(sepia[ChannelFlags::RGB]);

criterion_group!(benches, sepia_benchmark);
criterion_main!(benches);
