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
        detection::edge::canny::{
            canny,
            canny_par,
        },
        geometry::{
            crop::{
                crop,
                crop_par,
            },
            resize::{
                resize,
                resize_par,
            },
        },
    },
    pixel::PixelFlags,
    primitive::{
        margin::Margin,
        scale::Scale,
        size::Size,
    },
};
use paste::paste;

macro_rules! add_bench_for_size {
    ($operation_name:ident, $size:expr, $criterion:ident, $($args:expr),* ) => {
        paste! {
            {
                let [<image $size>] = black_box(Image::empty(Size::from_usize($size, $size).unwrap()));
                let mut group = $criterion.benchmark_group(stringify!([<$operation_name _ $size x $size>]));
                group.sample_size(50);
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

macro_rules! operation_bench {
    ($operation_name:ident [$($args:expr),*]) => {
        paste! {
            fn [<$operation_name _benchmark>](criterion: &mut Criterion) {
                add_bench_for_size!($operation_name, 25, criterion, $($args),*);
                add_bench_for_size!($operation_name, 250, criterion, $($args),*);
                add_bench_for_size!($operation_name, 2500, criterion, $($args),*);
            }
        }
    };
}

operation_bench!(grayscale[PixelFlags::RGB]);
operation_bench!(sepia[PixelFlags::RGB]);
operation_bench!(gamma_correction [0.4, PixelFlags::RGB]);

operation_bench!(mean_blur [2, PixelFlags::RGB]);
operation_bench!(gaussian_blur [2, 3.0, PixelFlags::RGB]);

operation_bench!(crop[Margin::unified(3)]);
operation_bench!(resize[Scale::new(0.5, 0.5).unwrap()]);

operation_bench!(canny []);

criterion_group!(
    color_operations,
    grayscale_benchmark,
    sepia_benchmark,
    gamma_correction_benchmark,
);

criterion_group!(blur_operations, mean_blur_benchmark, gaussian_blur_benchmark,);

criterion_group!(geometry_operations, crop_benchmark, resize_benchmark);

criterion_group!(detection_edge_operations, canny_benchmark);

criterion_main!(color_operations, blur_operations, geometry_operations, detection_edge_operations);
