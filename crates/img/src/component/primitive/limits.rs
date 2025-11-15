/// Maximum dimension size of an image (width or height).
/// Guaranted to be less than isize::MAX.
#[cfg(target_pointer_width = "64")]
pub const DIMENSION_MAX: usize = (1u64 << 32) as usize - 1;

#[cfg(target_pointer_width = "32")]
pub const DIMENSION_MAX: usize = (1u32 << 16) as usize - 1;

