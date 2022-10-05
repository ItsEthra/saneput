use std::{slice, io::{Stdin, Read, self}};
use crate::ParseIntError;

#[derive(Debug, Clone, Copy)]
pub enum ExpectedRadix {
    Bin,
    Oct,
    Dec,
    Hex,
}

pub trait FromStdin {
    type Output;
    type Error: From<io::Error>;

    fn read_cin(cin: &mut Stdin, radix: Option<ExpectedRadix>) -> Result<Self::Output, Self::Error>;
}

macro_rules! impl_from_cin_prim {
    ($($ty:ty),*) => {
        $(
            impl FromStdin for $ty {
                type Output = $ty;
                type Error = ParseIntError;

                fn read_cin(cin: &mut Stdin, radix: Option<ExpectedRadix>) -> Result<Self::Output, Self::Error> {
                    let (mut b, mut v, mut neg): (_, $ty, _) = (0, 0, None);

                    loop {
                        cin.read(slice::from_mut(&mut b))?;

                        if b == b' ' || b == b'\n' {
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
                                v = v.checked_add(if b >= b'0' && b <= b'7' {
                                    (b - b'0') as $ty
                                } else {
                                    return Err(ParseIntError::UnexpectedChar(b as char))
                                }).ok_or(ParseIntError::OutOfRange)?;
                            },
                            ExpectedRadix::Dec => {
                                v = v.checked_mul(10).ok_or(ParseIntError::OutOfRange)?;
                                v = v.checked_add(if b >= b'0' && b <= b'9' {
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

                        if v >= <$ty>::MAX || v < <$ty>::MIN {
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

impl_from_cin_prim!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

