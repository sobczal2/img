mod canny;

pub use canny::{
    canny,
    canny_lens,
};

#[cfg(feature = "parallel")]
pub use canny::{
    canny_lens_par,
    canny_par,
};
