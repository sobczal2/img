use crate::error::{IndexResult, OutOfBoundsError};

#[derive(Debug)]
pub struct Buffer(Box<[u8]>);

impl Buffer {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_data(&self, index: usize, length: usize) -> IndexResult<&[u8]> {
        if index + length > self.len() {
            return Err(OutOfBoundsError);
        }

        Ok(unsafe { self.get_data_unchecked(index, length) })
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics in debug if index + length <= buffer len.
    ///
    /// # Safety
    /// - index + length have to be within buffer size
    ///
    pub unsafe fn get_data_unchecked(&self, index: usize, length: usize) -> &[u8] {
        debug_assert!(index + length <= self.len(), "buffer out of bounds access");
        &self.0[index..index + length]
    }

    pub fn get_data_mut(&mut self, index: usize, length: usize) -> IndexResult<&mut [u8]> {
        if index + length > self.len() {
            return Err(OutOfBoundsError);
        }

        Ok(unsafe { self.get_data_mut_unchecked(index, length) })
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics in debug if index + length <= buffer len.
    ///
    /// # Safety
    /// - index + length have to be within buffer size
    ///
    pub unsafe fn get_data_mut_unchecked(&mut self, index: usize, length: usize) -> &mut [u8] {
        debug_assert!(index + length <= self.len(), "buffer out of bounds access");
        &mut self.0[index..index + length]
    }
}

impl FromIterator<u8> for Buffer {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Buffer(Box::from_iter(iter))
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
