use std::{slice, io::{Stdin, Read}, num::ParseFloatError};
use crate::ParseIntError;

/// Integer number system base.
#[derive(Debug, Clone, Copy)]
pub enum ExpectedRadix {
    /// Binary
    Bin,
    /// Octal
    Oct,
    /// Decimal
    Dec,
    /// Hexidecimal
    Hex,
}

/// Type that can be read from standrard input.
pub trait FromStdin: Sized {
    type Error: std::error::Error;

    /// Reads the value from standrard input with optional radix.
    fn read_cin(cin: &mut Stdin, radix: Option<ExpectedRadix>) -> Result<Self, Self::Error>;
}

macro_rules! impl_from_cin_prim {
    ($($ty:ty),*) => {
        $(
            impl FromStdin for $ty {
                type Error = ParseIntError;

                // Parsing done manually in effort to avoid allocation, not sure if it's the best
                // way to go about this.
                fn read_cin(cin: &mut Stdin, radix: Option<ExpectedRadix>) -> Result<Self, Self::Error> {
                    let (mut b, mut v, mut neg): (_, $ty, _) = (0, 0, None);

                    loop {
                        cin.read_exact(slice::from_mut(&mut b))?;

                        if b.is_ascii_control() || b == b' ' {
                            break;
                        } else if b == b'-' && neg.is_none() {
                            neg = Some(true);
                            continue;
                        } else if b == b'_' {
                            continue;
                        }

                        match radix.unwrap_or(ExpectedRadix::Dec) {
                            ExpectedRadix::Bin => {
                                v = v.checked_mul(2).ok_or(ParseIntError::OutOfRange)?;
                                v = v.checked_add(if b == b'1' || b == b'0' {
                                    (b == b'1') as $ty 
                                } else {
                                    return Err(ParseIntError::UnexpectedChar(b as char))
                                }).ok_or(ParseIntError::OutOfRange)?;
                            },
                            ExpectedRadix::Oct => {
                                v = v.checked_mul(8).ok_or(ParseIntError::OutOfRange)?;
                                v = v.checked_add(if (b'0'..=b'7').contains(&b) {
                                    (b - b'0') as $ty
                                } else {
                                    return Err(ParseIntError::UnexpectedChar(b as char))
                                }).ok_or(ParseIntError::OutOfRange)?;
                            },
                            ExpectedRadix::Dec => {
                                v = v.checked_mul(10).ok_or(ParseIntError::OutOfRange)?;
                                v = v.checked_add(if (b'0'..=b'9').contains(&b) {
                                    (b - b'0') as $ty 
                                } else {
                                    return Err(ParseIntError::UnexpectedChar(b as char))
                                }).ok_or(ParseIntError::OutOfRange)?;
                            },
                            ExpectedRadix::Hex => {
                                v = v.checked_mul(16).ok_or(ParseIntError::OutOfRange)?;
                                v = v.checked_add(match b {
                                    b'0'..=b'9' => (b - b'0') as $ty,
                                    b'a'..=b'f' => 10 + (b - b'a') as $ty,
                                    b'A'..=b'F' => 10 + (b - b'A') as $ty,
                                    _ => return Err(ParseIntError::UnexpectedChar(b as char))
                                }).ok_or(ParseIntError::OutOfRange)?;
                            },
                        }

                        if !(<$ty>::MIN..=<$ty>::MAX).contains(&v) {
                            return Err(ParseIntError::OutOfRange);
                        }
                    }

                    Ok(if matches!(neg, Some(true)) {
                        if <$ty>::MIN == 0 {
                            return Err(ParseIntError::NegUnsigned);
                        }

                        v.checked_neg().unwrap()
                    } else {
                        v
                    })
                }
            }
        )*
    };
}

impl_from_cin_prim!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

impl FromStdin for f32 {
    type Error = ParseFloatError;

    fn read_cin(cin: &mut Stdin, radix: Option<ExpectedRadix>) -> Result<Self, Self::Error> {
        assert!(radix.is_none(), "f32 does not accept radix argument");

        let mut buf = Vec::new();
        let mut b = 0;
        loop {
            cin.read_exact(slice::from_mut(&mut b)).unwrap();

            if b.is_ascii_control() || b == b' ' {
                break;
            }
            buf.push(b);
        }

        let s = String::from_utf8(buf).unwrap();
        Ok(s.parse()?)
    }
}

impl FromStdin for f64 {
    type Error = ParseFloatError;

    fn read_cin(cin: &mut Stdin, radix: Option<ExpectedRadix>) -> Result<Self, Self::Error> {
        assert!(radix.is_none(), "f64 does not accept radix argument");

        let mut buf = Vec::new();
        let mut b = 0;
        loop {
            cin.read_exact(slice::from_mut(&mut b)).unwrap();

            if b.is_ascii_control() || b == b' ' {
                break;
            }
            buf.push(b);
        }

        let s = String::from_utf8(buf).unwrap();
        Ok(s.parse()?)
    }
}

