use std::ops::{
    AddAssign,
    SubAssign,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HistogramError {
    #[error("index out of bounds")]
    OutOfBounds,
    #[error("counter overflow")]
    Overflow,
    #[error("counter underflow")]
    Underflow,
    #[error("histogram is empty")]
    Empty,
}

pub type HistogramResult<T> = Result<T, HistogramError>;

/// Trait restricting allowed counter types
pub trait Counter: Copy + Default + PartialOrd + AddAssign + SubAssign {
    fn checked_inc(self) -> Option<Self>;
    fn checked_dec(self) -> Option<Self>;
    fn half(self) -> Self;
}

macro_rules! impl_counter {
    ($t:ty) => {
        impl Counter for $t {
            fn checked_inc(self) -> Option<Self> {
                self.checked_add(1)
            }
            fn checked_dec(self) -> Option<Self> {
                self.checked_sub(1)
            }
            fn half(self) -> Self {
                self / 2
            }
        }
    };
}

impl_counter!(u8);
impl_counter!(u16);
impl_counter!(u32);
impl_counter!(u64);

#[derive(Debug)]
pub struct Histogram<C, const N: usize>
where
    C: Counter,
{
    bins: [C; N],
    total: C,
}

impl<C, const N: usize> Default for Histogram<C, N>
where
    C: Counter,
{
    fn default() -> Self {
        Self { bins: [C::default(); N], total: C::default() }
    }
}

impl<C, const N: usize> Histogram<C, N>
where
    C: Counter,
{
    pub fn insert_index(&mut self, index: usize) -> HistogramResult<()> {
        if index >= N {
            return Err(HistogramError::OutOfBounds);
        }

        self.bins[index] = self.bins[index].checked_inc().ok_or(HistogramError::Overflow)?;

        self.total = self.total.checked_inc().ok_or(HistogramError::Overflow)?;

        Ok(())
    }

    pub fn remove_index(&mut self, index: usize) -> HistogramResult<()> {
        if index >= N {
            return Err(HistogramError::OutOfBounds);
        }

        self.bins[index] = self.bins[index].checked_dec().ok_or(HistogramError::Underflow)?;

        self.total = self.total.checked_dec().ok_or(HistogramError::Underflow)?;

        Ok(())
    }

    pub fn reset(&mut self) {
        self.bins.fill(C::default());
        self.total = C::default();
    }

    pub fn median(&self) -> HistogramResult<usize> {
        if self.total == C::default() {
            return Err(HistogramError::Empty);
        }

        let mut cumulative = C::default();
        let half = {
            let mut h = self.total;
            h = h.half();
            h
        };

        for (i, &count) in self.bins.iter().enumerate() {
            cumulative += count;
            if cumulative > half {
                return Ok(i);
            }
        }

        unreachable!("should never happen if total is consistent")
    }
}
