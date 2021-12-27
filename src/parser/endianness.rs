use std::convert::TryFrom;

use super::TiffParserError;

#[derive(Debug, Clone, Copy)]
pub(super) enum Endianness {
    LittleEndian,
    BigEndian,
}

impl Endianness {
    pub(super) fn read_i16(&self, buf: &[u8]) -> Result<i16, TiffParserError> {
        let bytes = <[u8; 2]>::try_from(&buf[0..2])?;
        let val = match self {
            Endianness::LittleEndian => i16::from_le_bytes(bytes),
            Endianness::BigEndian => i16::from_be_bytes(bytes),
        };
        Ok(val)
    }

    pub(super) fn read_u16(&self, buf: &[u8]) -> Result<u16, TiffParserError> {
        let bytes = <[u8; 2]>::try_from(&buf[0..2])?;
        let val = match self {
            Endianness::LittleEndian => u16::from_le_bytes(bytes),
            Endianness::BigEndian => u16::from_be_bytes(bytes),
        };
        Ok(val)
    }

    pub(super) fn read_i32(&self, buf: &[u8]) -> Result<i32, TiffParserError> {
        let bytes = <[u8; 4]>::try_from(&buf[0..4])?;
        let val = match self {
            Endianness::LittleEndian => i32::from_le_bytes(bytes),
            Endianness::BigEndian => i32::from_be_bytes(bytes),
        };
        Ok(val)
    }

    pub(super) fn read_u32(&self, buf: &[u8]) -> Result<u32, TiffParserError> {
        let bytes = <[u8; 4]>::try_from(&buf[0..4])?;
        let val = match self {
            Endianness::LittleEndian => u32::from_le_bytes(bytes),
            Endianness::BigEndian => u32::from_be_bytes(bytes),
        };
        Ok(val)
    }

    pub(super) fn read_f32(&self, buf: &[u8]) -> Result<f32, TiffParserError> {
        let bytes = <[u8; 4]>::try_from(&buf[0..4])?;
        let val = match self {
            Endianness::LittleEndian => f32::from_le_bytes(bytes),
            Endianness::BigEndian => f32::from_be_bytes(bytes),
        };
        Ok(val)
    }

    pub(super) fn read_f64(&self, buf: &[u8]) -> Result<f64, TiffParserError> {
        let bytes = <[u8; 8]>::try_from(&buf[0..8])?;
        let val = match self {
            Endianness::LittleEndian => f64::from_le_bytes(bytes),
            Endianness::BigEndian => f64::from_be_bytes(bytes),
        };
        Ok(val)
    }
}
