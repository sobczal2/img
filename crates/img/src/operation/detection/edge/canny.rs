use thiserror::Error;

use crate::{
    component::kernel::{
        gaussian::GaussianKernel,
        sobel::SobelKernel,
    },
    pipe::{
        Pipe,
        image::PixelPipe,
        kernel,
    },
    pixel::{
        Pixel,
        PixelFlags,
    },
    primitive::size::Size,
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("kernel creation error: {0}")]
    KernelCreationError(#[from] kernel::CreationError),
}

pub type CreationResult<T> = std::result::Result<T, CreationError>;

// TODO: finish
pub fn canny_pipe<S>(source: S) -> Result<impl Pipe<Item = Pixel>, CreationError>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    let pipe = source
        // .kernel(GaussianKernel::new(Size::from_radius(2), 3f32, PixelFlags::RGB).unwrap())?
        .colors(
            |red| {
                CreationResult::Ok(
                    red.kernel(SobelKernel::new())?
                        .map(|gradient| gradient.magnitude())
                        .map(|magnitude| if magnitude >= 10f32 { 255u8 } else { 0u8 }),
                )
            },
            |blue| {
                CreationResult::Ok(
                    blue.kernel(SobelKernel::new())?
                        .map(|gradient| gradient.magnitude())
                        .map(|magnitude| if magnitude >= 10f32 { 255u8 } else { 0u8 }),
                )
            },
            |green| {
                CreationResult::Ok(
                    green
                        .kernel(SobelKernel::new())?
                        .map(|gradient| gradient.magnitude())
                        .map(|magnitude| if magnitude >= 10f32 { 255u8 } else { 0u8 }),
                )
            },
            CreationResult::Ok,
        )?;

    Ok(pipe)
}
