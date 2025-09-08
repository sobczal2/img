mod gamma_correction;
mod grayscale;
mod sepia;

pub use gamma_correction::{
    gamma_correction,
    gamma_correction_lens,
};
pub use grayscale::{
    grayscale,
    grayscale_lens,
};
pub use sepia::{
    sepia,
    sepia_lens,
};

#[cfg(feature = "parallel")]
pub use self::{
    gamma_correction::gamma_correction_par,
    grayscale::grayscale_par,
    sepia::sepia_par,
};
