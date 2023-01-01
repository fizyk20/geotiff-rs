use std::fmt;

use super::{
    compression::create_decompressor, endianness::Endianness, field::Field, tags::*, value::Value,
    TiffParserError,
};

#[derive(Debug)]
pub struct IfdEntry {
    pub(super) tag: u16,
    pub(super) value: Value,
}

impl IfdEntry {
    fn read(endianness: Endianness, buf: &[u8], start: usize) -> Result<Self, TiffParserError> {
        let tag = endianness.read_u16(&buf[start..])?;
        let field = Field::from_u16(endianness.read_u16(&buf[start + 2..])?)?;
        let count = endianness.read_u32(&buf[start + 4..])? as usize;
        let num_bytes = count * field.size();
        let bytes = if num_bytes <= 4 {
            &buf[start + 8..start + 8 + num_bytes]
        } else {
            let offset = endianness.read_u32(&buf[start + 8..])? as usize;
            &buf[offset..offset + num_bytes]
        };
        let value = Value::from_bytes(endianness, field, bytes)?;
        Ok(IfdEntry { tag, value })
    }
}

pub struct Ifd {
    pub entries: Vec<IfdEntry>,
    pub(super) sub_ifds: Vec<Ifd>,
    pub(crate) data: Vec<i32>,
}

impl Ifd {
    pub(super) fn read(
        endianness: Endianness,
        buf: &[u8],
        start: usize,
    ) -> Result<(Self, usize), TiffParserError> {
        let num_entries = endianness.read_u16(&buf[start..])? as usize;
        let mut entries = vec![];
        let mut sub_ifds = vec![];
        for i in 0..num_entries {
            let entry = IfdEntry::read(endianness, buf, start + 2 + i * 12)?;
            match (&entry.tag, &entry.value) {
                (&TAG_SUB_IFDS, &Value::Longs(ref longs)) => {
                    let mut offset = longs[0] as usize;
                    while offset != 0 {
                        let (sub_ifd, next_offset) = Ifd::read(endianness, buf, offset)?;
                        sub_ifds.push(sub_ifd);
                        offset = next_offset;
                    }
                }
                _ => {
                    entries.push(entry);
                }
            }
        }
        let next_ifd_offset = endianness.read_u32(&buf[start + 2 + num_entries * 12..])? as usize;

        let mut ifd = Ifd {
            entries,
            sub_ifds,
            data: vec![],
        };

        ifd.read_data(endianness, buf)?;

        Ok((ifd, next_ifd_offset))
    }

    pub fn image_width(&self) -> Option<u16> {
        self.get_value(TAG_IMAGE_WIDTH).ok()?.short().ok()
    }

    pub fn image_length(&self) -> Option<u16> {
        self.get_value(TAG_IMAGE_LENGTH).ok()?.short().ok()
    }

    fn read_data(&mut self, endianness: Endianness, buf: &[u8]) -> Result<(), TiffParserError> {
        if self.is_stripped() && !self.is_tiled() {
            self.read_data_stripped(endianness, buf)
        } else if self.is_tiled() && !self.is_stripped() {
            self.read_data_tiled(endianness, buf)
        } else if self.is_tiled() && self.is_stripped() {
            Err(TiffParserError::ImageBothTiledAndStripped)
        } else {
            // no image data
            Ok(())
        }
    }

    fn read_data_stripped(
        &mut self,
        _endianness: Endianness,
        _buf: &[u8],
    ) -> Result<(), TiffParserError> {
        unimplemented!()
    }

    fn read_data_tiled(
        &mut self,
        endianness: Endianness,
        buf: &[u8],
    ) -> Result<(), TiffParserError> {
        let tile_width = self.get_value(TAG_TILE_WIDTH)?.short()? as usize;
        let tile_length = self.get_value(TAG_TILE_LENGTH)?.short()? as usize;
        let tile_offsets = self.get_value(TAG_TILE_OFFSETS)?.longs()?;
        let tile_byte_counts = self.get_value(TAG_TILE_BYTE_COUNTS)?.longs()?;

        let image_width = self.get_value(TAG_IMAGE_WIDTH)?.short()? as usize;
        let image_length = self.get_value(TAG_IMAGE_LENGTH)?.short()? as usize;

        let bits_per_sample = self.get_value(TAG_BITS_PER_SAMPLE)?.short()? as usize;
        let samples_per_pixel = self.get_value(TAG_SAMPLES_PER_PIXEL)?.short()? as usize;
        let bytes_per_pixel = bits_per_sample * samples_per_pixel / 8;

        let compression = self.get_value(TAG_COMPRESSION)?.short()?;

        let tiles = tile_offsets
            .iter()
            .zip(tile_byte_counts.iter())
            .map(|(offset, count)| {
                let offset = *offset as usize;
                let count = *count as usize;
                &buf[offset..offset + count]
            })
            .map(|enc_tile| {
                create_decompressor(compression).and_then(|mut decompressor| {
                    decompressor
                        .decompress(enc_tile, tile_width * tile_length * bits_per_sample / 8)
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.data = Vec::with_capacity(image_length * image_width);
        let nrow = (image_width + tile_width - 1) / tile_width;
        for j in 0..image_length {
            let tile_row = j / tile_length;
            let jt = j - tile_row * tile_length;
            for i in 0..image_width {
                let tile_col = i / tile_width;
                let it = i - tile_col * tile_width;
                let bytes =
                    &tiles[tile_row * nrow + tile_col][bytes_per_pixel * (jt * tile_width + it)..];
                let pixel = match bytes_per_pixel {
                    2 => endianness.read_i16(bytes)? as i32,
                    4 => endianness.read_i32(bytes)?,
                    _ => unimplemented!(),
                };
                self.data.push(pixel);
            }
        }

        Ok(())
    }

    fn is_stripped(&self) -> bool {
        self.has_entry(TAG_ROWS_PER_STRIP)
            && self.has_entry(TAG_STRIP_OFFSETS)
            && self.has_entry(TAG_STRIP_BYTE_COUNTS)
    }

    fn is_tiled(&self) -> bool {
        self.has_entry(TAG_TILE_WIDTH)
            && self.has_entry(TAG_TILE_LENGTH)
            && self.has_entry(TAG_TILE_OFFSETS)
            && self.has_entry(TAG_TILE_BYTE_COUNTS)
    }

    pub fn has_entry(&self, tag: u16) -> bool {
        self.entries.iter().any(|entry| entry.tag == tag)
    }

    #[allow(unused)]
    pub fn has_entry_recursive(&self, tag: u16) -> bool {
        self.has_entry(tag)
            || self
                .sub_ifds
                .iter()
                .any(|sub_ifd| sub_ifd.has_entry_recursive(tag))
    }

    pub fn get_value(&self, tag: u16) -> Result<&Value, TiffParserError> {
        self.entries
            .iter()
            .find(|entry| entry.tag == tag)
            .map(|entry| &entry.value)
            .ok_or(TiffParserError::MissingValue(tag))
    }

    #[allow(unused)]
    pub fn get_value_recursive(&self, tag: u16) -> Result<&Value, TiffParserError> {
        self.get_value(tag)
            .ok()
            .or_else(|| {
                self.sub_ifds
                    .iter()
                    .filter_map(|sub_ifd| sub_ifd.get_value(tag).ok())
                    .next()
            })
            .ok_or(TiffParserError::MissingValue(tag))
    }
}

impl fmt::Debug for Ifd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Ifd")
            .field("entries", &self.entries)
            .field("sub_ifds", &self.sub_ifds)
            .field("data", &format!("({} pixels)", self.data.len()))
            .finish()
    }
}
