use std::path::Path;

use super::{TiffFile, TiffParserError};

#[derive(Debug)]
pub struct GeoTiff {
    tiff: TiffFile,
}

impl GeoTiff {
    pub fn from_file<P: AsRef<Path>>(name: P) -> Result<Self, TiffParserError> {
        let tiff = TiffFile::from_file(name)?;

        Ok(Self { tiff })
    }

    pub fn get_pixel(&self, lon: usize, lat: usize) -> i32 {
        let ifd = &self.tiff.ifds[0];
        let width = ifd.image_width().unwrap() as usize;
        let length = ifd.image_length().unwrap() as usize;
        ifd.data[(length - 1 - lat) * width + lon]
    }
}
