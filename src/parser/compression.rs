use weezl::{decode::Decoder, BitOrder};

use super::TiffParserError;

pub trait Decompressor {
    fn decompress(&mut self, bytes: &[u8]) -> Result<Vec<u8>, TiffParserError>;
}

const COMPRESSION_NONE: u16 = 1;
const COMPRESSION_LZW: u16 = 5;

pub fn create_decompressor(compression: u16) -> Result<Box<dyn Decompressor>, TiffParserError> {
    match compression {
        COMPRESSION_NONE => Ok(Box::new(DummyDecompressor)),
        COMPRESSION_LZW => Ok(Box::new(Decoder::with_tiff_size_switch(BitOrder::Msb, 8))),
        compression => Err(TiffParserError::UnknownCompression(compression)),
    }
}

struct DummyDecompressor;

impl Decompressor for DummyDecompressor {
    fn decompress(&mut self, bytes: &[u8]) -> Result<Vec<u8>, TiffParserError> {
        Ok(bytes.to_vec())
    }
}

impl Decompressor for Decoder {
    fn decompress(&mut self, bytes: &[u8]) -> Result<Vec<u8>, TiffParserError> {
        Ok(self.decode(bytes)?)
    }
}
