use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use img::{
    image::Image,
    operation::{
        blur::{
            gaussian::{
                gaussian_blur,
                gaussian_blur_par,
            },
            mean::{
                mean_blur,
                mean_blur_par,
            },
        },
        color::{
            gamma_correction::{
                gamma_correction,
                gamma_correction_par,
            },
            grayscale::{
                grayscale,
                grayscale_par,
            },
            sepia::{
                sepia,
                sepia_par,
            },
        },
    },
    pixel::PixelFlags,
    primitive::size::Size,
};
use paste::paste;

macro_rules! add_bench_for_size {
    ($operation_name:ident, $size:expr, $criterion:ident, $($args:expr),* ) => {
        paste! {
            {
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
            }
        }
    }
}

macro_rules! operation_bench {
    ($operation_name:ident, $($args:expr),* ) => {
        paste! {
            fn [<$operation_name _benchmark>](criterion: &mut Criterion) {
                add_bench_for_size!($operation_name, 10, criterion, $($args),*);
                add_bench_for_size!($operation_name, 100, criterion, $($args),*);
                add_bench_for_size!($operation_name, 1000, criterion, $($args),*);
            }
        }
    };
}

operation_bench!(grayscale, PixelFlags::RGB);
operation_bench!(sepia, PixelFlags::RGB);
operation_bench!(gamma_correction, 0.4, PixelFlags::RGB);

operation_bench!(mean_blur, 2, PixelFlags::RGB);
operation_bench!(gaussian_blur, 2, 3.0, PixelFlags::RGB);

criterion_group!(
    color_operations,
    grayscale_benchmark,
    sepia_benchmark,
    gamma_correction_benchmark,
);

criterion_group!(blur_operations, mean_blur_benchmark, gaussian_blur_benchmark,);

criterion_main!(color_operations, blur_operations);
