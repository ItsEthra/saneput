use std::{error, fmt::{Display, self}, io};

/// Error that can occur when parsing an integer.
#[non_exhaustive]
#[derive(Debug)]
pub enum ParseIntError {
    /// Tried to negate an unsigned value.
    NegUnsigned,
    /// Parsed value exeeds type's representable range.
    OutOfRange,
    /// Found an unexpected character.
    UnexpectedChar(char),
    /// Input/Output error.
    Io(io::Error),
}

impl From<io::Error> for ParseIntError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegUnsigned => write!(f, "Expected unsigned but found negative value"),
            Self::OutOfRange => write!(f, "Value is out of representable range"),
            Self::UnexpectedChar(c) => write!(f, "Unexpected character `{c}`"),
            Self::Io(e) => e.fmt(f),
        }
    }
}
impl error::Error for ParseIntError {}

