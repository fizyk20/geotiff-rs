use std::{fs::File, io::Read, path::Path};

use super::{endianness::Endianness, ifd::Ifd, TiffParserError};

#[derive(Debug)]
pub struct TiffFile {
    pub(crate) ifds: Vec<Ifd>,
}

impl TiffFile {
    pub fn from_file<P: AsRef<Path>>(name: P) -> Result<Self, TiffParserError> {
        let mut file = File::open(name)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;

        Self::from_bytes(&buffer)
    }

    pub fn from_bytes(buf: &[u8]) -> Result<TiffFile, TiffParserError> {
        let endianness = match &buf[0..2] {
            b"II" => Endianness::LittleEndian,
            b"MM" => Endianness::BigEndian,
            marker => {
                return Err(TiffParserError::UnknownEndiannessMarker(marker.to_vec()));
            }
        };

        let mut next_ifd_offset = endianness.read_u32(&buf[4..])? as usize;
        let mut ifds = vec![];

        while next_ifd_offset != 0 {
            let (ifd, offset) = Ifd::read(endianness, buf, next_ifd_offset)?;
            ifds.push(ifd);
            next_ifd_offset = offset;
        }

        let tiff = TiffFile { ifds };

        Ok(tiff)
    }
}
