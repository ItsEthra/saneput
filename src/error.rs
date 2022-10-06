use std::{error, fmt::{Display, self}, io, string::FromUtf8Error};

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

#[non_exhaustive]
#[derive(Debug)]
pub enum ParseFloatError {
    /// Float parsing from string gone wrong.
    Parse(std::num::ParseFloatError),
    /// Input/Output error.
    Io(io::Error),
    /// Float string was not a utf8 sequence.
    NotUtf8(FromUtf8Error),
}

impl From<io::Error> for ParseFloatError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<FromUtf8Error> for ParseFloatError {
    fn from(value: FromUtf8Error) -> Self {
        Self::NotUtf8(value)
    }
}

impl From<std::num::ParseFloatError> for ParseFloatError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::Parse(value)
    }
}

impl Display for ParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => e.fmt(f),
            Self::NotUtf8(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
        }
    }
}
impl error::Error for ParseFloatError {}
