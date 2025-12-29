mod gaussian;
mod kuwahara;
mod mean;
mod median;

pub use gaussian::{
    gaussian_blur,
    gaussian_blur_lens,
};
pub use kuwahara::{
    kuwahara,
    kuwahara_lens,
};
pub use mean::{
    mean_blur,
    mean_blur_lens,
};
pub use median::{
    median_blur,
    median_blur_lens,
};

#[cfg(feature = "parallel")]
pub use self::{
    gaussian::gaussian_blur_par,
    kuwahara::kuwahara_par,
    mean::mean_blur_par,
    median::median_blur_par,
};
