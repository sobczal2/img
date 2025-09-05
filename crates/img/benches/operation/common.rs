#[macro_export]
macro_rules! add_bench_for_size {
    ($operation_name:ident, $size:expr, $criterion:ident, $($args:expr),* ) => {
        paste! {
            {
                use img::{image::Image, primitive::size::Size};

                let [<image $size>] = black_box(Image::empty(Size::from_usize($size, $size).unwrap()));
                let mut group = $criterion.benchmark_group(stringify!([<$operation_name _ $size x $size>]));
                group.bench_function(
                    "sequential",
                    |b| b.iter(|| $operation_name(&[<image $size>], $($args),*))
                );
                group.bench_function(
                    "parallel",
                    |b| b.iter(|| [<$operation_name _par>](&[<image $size>], $($args),*))
                );
                group.finish();
            }
        }
    }
}

#[macro_export]
macro_rules! operation_bench {
    ($operation_name:ident [$($args:expr),*]) => {
        use paste::paste;
        paste! {
            fn [<$operation_name _benchmark>](criterion: &mut Criterion) {
                add_bench_for_size!($operation_name, 25, criterion, $($args),*);
                add_bench_for_size!($operation_name, 250, criterion, $($args),*);
                add_bench_for_size!($operation_name, 2500, criterion, $($args),*);
            }
        }
    };
}
