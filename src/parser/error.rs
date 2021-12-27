use std::{array::TryFromSliceError, io::Error as IoError, string::FromUtf8Error};

use thiserror::Error;
use weezl::LzwError;

use super::value::Value;

#[derive(Debug, Error)]
pub enum TiffParserError {
    #[error("Converting slice to array failed: {0}")]
    TryFromSlice(#[from] TryFromSliceError),
    #[error("UTF-8 decoding failed: {0}")]
    Utf8(#[from] FromUtf8Error),
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    #[error("A string wasn't null-terminated: {0:?}")]
    StringNotNullTerminated(Vec<u8>),
    #[error("Unknown field type: {0}")]
    UnknownFieldType(u16),
    #[error("Unknown endianness marker: {0:?}")]
    UnknownEndiannessMarker(Vec<u8>),
    #[error("The image has both tile and strip data")]
    ImageBothTiledAndStripped,
    #[error("Invalid value {0:?}, {1}")]
    InvalidValue(Value, &'static str),
    #[error("Missing value for tag {0}")]
    MissingValue(u16),
    #[error("Unknown compression scheme: {0}")]
    UnknownCompression(u16),
    #[error("LZW decompression error: {0}")]
    Lzw(#[from] LzwError),
}
