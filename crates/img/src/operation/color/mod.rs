mod brightness;
mod gamma_correction;
mod grayscale;
mod negative;
mod sepia;

pub use brightness::{brightness, brightness_lens};
pub use gamma_correction::{gamma_correction, gamma_correction_lens};
pub use grayscale::{grayscale, grayscale_lens};
pub use negative::{negative, negative_lens};
pub use sepia::{sepia, sepia_lens};

#[cfg(feature = "parallel")]
pub use self::{
    brightness::brightness_par, gamma_correction::gamma_correction_par, grayscale::grayscale_par,
    negative::negative_par, sepia::sepia_par,
};
