mod compression;
mod endianness;
mod error;
mod field;
mod ifd;
mod tags;
mod tiff_file;
mod value;

pub use error::TiffParserError;
pub use tiff_file::TiffFile;
