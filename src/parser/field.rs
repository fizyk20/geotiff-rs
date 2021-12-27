use super::TiffParserError;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Field {
    Byte = 1,
    Ascii = 2,
    Short = 3,
    Long = 4,
    Rational = 5,
    Sbyte = 6,
    Undefined = 7,
    Sshort = 8,
    Slong = 9,
    Srational = 10,
    Float = 11,
    Double = 12,
}

impl Field {
    pub(super) fn from_u16(field: u16) -> Result<Self, TiffParserError> {
        match field {
            1 => Ok(Field::Byte),
            2 => Ok(Field::Ascii),
            3 => Ok(Field::Short),
            4 => Ok(Field::Long),
            5 => Ok(Field::Rational),
            6 => Ok(Field::Sbyte),
            7 => Ok(Field::Undefined),
            8 => Ok(Field::Sshort),
            9 => Ok(Field::Slong),
            10 => Ok(Field::Srational),
            11 => Ok(Field::Float),
            12 => Ok(Field::Double),
            field => Err(TiffParserError::UnknownFieldType(field)),
        }
    }

    pub(super) fn size(&self) -> usize {
        match self {
            Field::Byte | Field::Ascii | Field::Sbyte | Field::Undefined => 1,
            Field::Short | Field::Sshort => 2,
            Field::Long | Field::Slong | Field::Float => 4,
            Field::Rational | Field::Srational | Field::Double => 8,
        }
    }
}
