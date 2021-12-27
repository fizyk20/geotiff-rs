use std::fmt;

use super::{endianness::Endianness, field::Field, TiffParserError};

#[derive(Clone)]
pub enum Value {
    Bytes(Vec<u8>),
    Sbytes(Vec<i8>),
    Shorts(Vec<u16>),
    Sshorts(Vec<i16>),
    Longs(Vec<u32>),
    Slongs(Vec<i32>),
    Rationals(Vec<(u32, u32)>),
    Srationals(Vec<(i32, i32)>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
    Ascii(String),
    Undefined(Vec<u8>),
}

fn try_tuple<T1, T2, E>(res1: Result<T1, E>, res2: Result<T2, E>) -> Result<(T1, T2), E> {
    let r1 = res1?;
    let r2 = res2?;
    Ok((r1, r2))
}

impl Value {
    pub(super) fn from_bytes(
        endianness: Endianness,
        field: Field,
        buf: &[u8],
    ) -> Result<Self, TiffParserError> {
        match field {
            Field::Byte => {
                let bytes = buf.to_vec();
                Ok(Value::Bytes(bytes))
            }
            Field::Sbyte => {
                let vals = buf.iter().map(|b| *b as i8).collect();
                Ok(Value::Sbytes(vals))
            }
            Field::Short => {
                let vals: Result<Vec<_>, _> =
                    buf.chunks(2).map(|b| endianness.read_u16(b)).collect();
                Ok(Value::Shorts(vals?))
            }
            Field::Sshort => {
                let vals: Result<Vec<_>, _> =
                    buf.chunks(2).map(|b| endianness.read_i16(b)).collect();
                Ok(Value::Sshorts(vals?))
            }
            Field::Long => {
                let vals: Result<Vec<_>, _> =
                    buf.chunks(4).map(|b| endianness.read_u32(b)).collect();
                Ok(Value::Longs(vals?))
            }
            Field::Slong => {
                let vals: Result<Vec<_>, _> =
                    buf.chunks(4).map(|b| endianness.read_i32(b)).collect();
                Ok(Value::Slongs(vals?))
            }
            Field::Rational => {
                let vals: Result<Vec<_>, _> = buf
                    .chunks(8)
                    .map(|b| try_tuple(endianness.read_u32(&b[0..]), endianness.read_u32(&b[4..])))
                    .collect();
                Ok(Value::Rationals(vals?))
            }
            Field::Srational => {
                let vals: Result<Vec<_>, _> = buf
                    .chunks(8)
                    .map(|b| try_tuple(endianness.read_i32(&b[0..]), endianness.read_i32(&b[4..])))
                    .collect();
                Ok(Value::Srationals(vals?))
            }
            Field::Ascii => {
                let bytes = buf.to_vec();
                let len = bytes.len();
                if bytes[len - 1] != 0 {
                    return Err(TiffParserError::StringNotNullTerminated(bytes));
                }
                Ok(Value::Ascii(String::from_utf8(bytes[0..len - 1].to_vec())?))
            }
            Field::Undefined => {
                let bytes = buf.to_vec();
                Ok(Value::Undefined(bytes))
            }
            Field::Float => {
                let vals: Result<Vec<_>, _> = buf
                    .chunks(4)
                    .map(|b| endianness.read_f32(&b[0..]))
                    .collect();
                Ok(Value::Floats(vals?))
            }
            Field::Double => {
                let vals: Result<Vec<_>, _> = buf
                    .chunks(8)
                    .map(|b| endianness.read_f64(&b[0..]))
                    .collect();
                Ok(Value::Doubles(vals?))
            }
        }
    }

    pub fn short(&self) -> Result<u16, TiffParserError> {
        match self {
            Value::Shorts(vals) => Ok(vals[0]),
            val => Err(TiffParserError::InvalidValue(val.clone(), "expected short")),
        }
    }

    pub fn long(&self) -> Result<u32, TiffParserError> {
        match self {
            Value::Longs(vals) => Ok(vals[0]),
            val => Err(TiffParserError::InvalidValue(val.clone(), "expected long")),
        }
    }

    pub fn shorts(&self) -> Result<&[u16], TiffParserError> {
        match self {
            Value::Shorts(vals) => Ok(vals),
            val => Err(TiffParserError::InvalidValue(
                val.clone(),
                "expected shorts",
            )),
        }
    }

    pub fn longs(&self) -> Result<&[u32], TiffParserError> {
        match self {
            Value::Longs(vals) => Ok(vals),
            val => Err(TiffParserError::InvalidValue(val.clone(), "expected longs")),
        }
    }
}

const MAX_LEN: usize = 226;

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bytes(bytes) => {
                if bytes.len() < MAX_LEN {
                    f.debug_tuple("Bytes").field(bytes).finish()
                } else {
                    f.debug_tuple("Bytes")
                        .field(&format_args!("{} bytes", bytes.len()))
                        .finish()
                }
            }
            Value::Sbytes(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Sbytes").field(vals).finish()
                } else {
                    f.debug_tuple("Sbytes")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Shorts(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Shorts").field(vals).finish()
                } else {
                    f.debug_tuple("Shorts")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Sshorts(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Sshorts").field(vals).finish()
                } else {
                    f.debug_tuple("Sshorts")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Longs(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Longs").field(vals).finish()
                } else {
                    f.debug_tuple("Longs")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Slongs(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Slongs").field(vals).finish()
                } else {
                    f.debug_tuple("Slongs")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Rationals(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Rationals").field(vals).finish()
                } else {
                    f.debug_tuple("Rationals")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Srationals(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Srationals").field(vals).finish()
                } else {
                    f.debug_tuple("Srationals")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Undefined(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Undefined").field(vals).finish()
                } else {
                    f.debug_tuple("Undefined")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Ascii(string) => write!(f, "\"{}\"", string),
            Value::Floats(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Floats").field(vals).finish()
                } else {
                    f.debug_tuple("Floats")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
            Value::Doubles(vals) => {
                if vals.len() < MAX_LEN {
                    f.debug_tuple("Doubles").field(vals).finish()
                } else {
                    f.debug_tuple("Doubles")
                        .field(&format_args!("{} values", vals.len()))
                        .finish()
                }
            }
        }
    }
}
