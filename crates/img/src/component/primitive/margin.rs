/// Represents a 2D margin with top, right, bottom, left non-negative integer values.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Margin {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

impl Margin {

    /// Create a new [`Margin`] with specified top, right, bottom, left components.
    ///
    /// # Examples
    ///
    /// ```
    /// use img::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let margin = Margin::new(100, 200, 300, 400);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        Margin { top, right, bottom, left }
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
    pub fn unified(value: usize) -> Self {
        Margin { top: value, right: value, bottom: value, left: value }
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
