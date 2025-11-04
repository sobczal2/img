use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::prelude::*;

mod common;

operation_bench!(negative[ChannelFlags::RGB]);

criterion_group!(benches, negative_benchmark);
criterion_main!(benches);
