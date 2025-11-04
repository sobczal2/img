use std::io;

use thiserror::Error;

/// Out of bounds error, may occur when trying
/// to access image pixel by index
#[derive(Debug, Error, PartialEq, Eq)]
#[error("out of bounds")]
pub struct OutOfBoundsError;

pub type IndexResult<T> = std::result::Result<T, OutOfBoundsError>;

/// Enum to facilitate different kinds of errors that may occur
/// when reading or writing images
#[derive(Debug, Error)]
pub enum IoError {
    #[error("png decoding error: {0}")]
    PngDecoding(#[from] png::DecodingError),
    #[error("jpeg decoding error: {0}")]
    JpegDecoding(turbojpeg::Error),
    #[error("jpeg decoding error: {0}")]
    JpegEncoding(turbojpeg::Error),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error("unexpected: {0}")]
    Unexpected(String),
}

pub type IoResult<T> = std::result::Result<T, IoError>;

/// Error occuring when size of a buffer is incorrect
/// according to image size
#[derive(Debug, Error)]
#[error("buffer length mismatch")]
pub struct BufferLengthMismatchError;

pub type BufferLengthMismatchResult<T> = std::result::Result<T, BufferLengthMismatchError>;
