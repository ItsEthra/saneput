#![doc = include_str!("../README.md")]

pub use saneput_proc::{input, input_checked};

use std::{
    num::{ParseFloatError, ParseIntError},
    string::FromUtf8Error,
    io::{Read, Stdin},
};

/// Type that can be read from standrard input.
pub trait FromStdin: Sized {
    type Error: std::error::Error;

    /// Reads the value from standrard input with optional radix.
    fn read_cin(cin: &mut Stdin, radix: Option<u32>) -> Result<Self, Self::Error>;
}

impl FromStdin for String {
    type Error = FromUtf8Error;

    fn read_cin(cin: &mut Stdin, radix: Option<u32>) -> Result<Self, Self::Error> {
        assert_eq!(radix, None, "String does not accept a radix argument");

        String::from_utf8(copy_til_whitespace(cin))
    }
}

macro_rules! impl_from_cin_prim {
    ($($ty:ty),*) => {
        $(
            impl FromStdin for $ty {
                type Error = ParseIntError;

                // Parsing done manually in effort to avoid allocation, not sure if it's the best
                // way to go about this.
                fn read_cin(cin: &mut Stdin, radix: Option<u32>) -> Result<Self, Self::Error> {
                    let buf = $crate::copy_til_whitespace(cin);
                    <$ty>::from_str_radix(&String::from_utf8_lossy(&buf[..]), radix.unwrap_or(10))
                }
            }
        )*
    };
}

macro_rules! impl_from_cin_float {
    ($($ty:ty),*) => {
        $(
            impl FromStdin for $ty {
                type Error = ParseFloatError;

                fn read_cin(cin: &mut Stdin, radix: Option<u32>) -> Result<Self, Self::Error> {
                    assert!(radix.is_none(), concat!(stringify!($ty), " does not accept radix argument"));

                    let buf = $crate::copy_til_whitespace(cin);
                    String::from_utf8_lossy(&buf[..]).parse()
                }
            }
        )*
    };
}

impl_from_cin_prim!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
impl_from_cin_float!(f32, f64);

fn copy_til_whitespace(cin: &mut Stdin) -> Vec<u8> {
    cin.bytes()
        .flatten()
        .take_while(|b| !b.is_ascii_whitespace() && !b.is_ascii_control())
        .collect()
}
