use thiserror::Error;

use crate::image::DIMENSION_MAX;

/// Represents a 2D margin with top, right, bottom, left non-negative integer values.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Margin {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum MarginCreationError {
    #[error("top too big")]
    TopTooBig,
    #[error("right too big")]
    RightTooBig,
    #[error("bottom too big")]
    BottomTooBig,
    #[error("left too big")]
    LeftTooBig,
}

pub type MarginCreationResult<T> = std::result::Result<T, MarginCreationError>;

impl Margin {
    /// Create a new [`Margin`] with specified top, right, bottom, left components.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let margin = Margin::new(100, 200, 300, 400)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> MarginCreationResult<Self> {
        if top > DIMENSION_MAX {
            return Err(MarginCreationError::TopTooBig)
        }

        if right > DIMENSION_MAX {
            return Err(MarginCreationError::RightTooBig)
        }

        if bottom > DIMENSION_MAX {
            return Err(MarginCreationError::BottomTooBig)
        }

        if left > DIMENSION_MAX {
            return Err(MarginCreationError::LeftTooBig)
        }

        Ok(Margin { top, right, bottom, left })
    }

    /// Create a new [`Margin`] with all components equal to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let margin = Margin::unified(100);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn unified(value: usize) -> MarginCreationResult<Self> {
        Margin::new(value, value, value, value)
    }

    /// Returns [`Margin`]'s top component.
    pub fn top(&self) -> usize {
        self.top
    }

    /// Returns [`Margin`]'s right component.
    pub fn right(&self) -> usize {
        self.right
    }

    /// Returns [`Margin`]'s bottom component.
    pub fn bottom(&self) -> usize {
        self.bottom
    }

    /// Returns [`Margin`]'s left component.
    pub fn left(&self) -> usize {
        self.left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ok() {
        assert!(Margin::new(0, 0, 0, 0).is_ok());
        assert!(Margin::new(DIMENSION_MAX, 0, 0, 0).is_ok());
        assert!(Margin::new(0, DIMENSION_MAX, 0, 0).is_ok());
        assert!(Margin::new(0, 0, DIMENSION_MAX, 0).is_ok());
        assert!(Margin::new(0, 0, 0, DIMENSION_MAX).is_ok());
    }
}
