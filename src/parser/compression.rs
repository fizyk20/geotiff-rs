use weezl::{decode::Decoder, BitOrder, LzwStatus};

use super::TiffParserError;

pub trait Decompressor {
    fn decompress(&mut self, bytes: &[u8], size: usize) -> Result<Vec<u8>, TiffParserError>;
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
    fn decompress(&mut self, bytes: &[u8], size: usize) -> Result<Vec<u8>, TiffParserError> {
        Ok(bytes[..size].to_vec())
    }
}

impl Decompressor for Decoder {
    fn decompress(&mut self, bytes: &[u8], size: usize) -> Result<Vec<u8>, TiffParserError> {
        let mut result = vec![0; size];
        let mut consumed_in = 0;
        let mut consumed_out = 0;
        loop {
            let decode_result =
                self.decode_bytes(&bytes[consumed_in..], &mut result[consumed_out..]);
            consumed_in += decode_result.consumed_in;
            consumed_out += decode_result.consumed_out;
            match decode_result.status {
                Ok(LzwStatus::Ok) => {}
                Ok(LzwStatus::NoProgress) => {
                    return Ok(result);
                }
                Ok(LzwStatus::Done) => {
                    return Ok(result);
                }
                Err(err) => {
                    eprintln!(
                        "error {}; size = {}, consumed_out = {}",
                        err, size, consumed_out
                    );
                    if consumed_out >= size {
                        return Ok(result);
                    } else {
                        return Err(err.into());
                    }
                }
            }
            if consumed_out >= size {
                return Ok(result);
            }
        }
    }
}
