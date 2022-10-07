#![doc = include_str!("../README.md")]

pub use saneput_proc::input;

use std::{slice, io::{Stdin, Read}, num::{ParseIntError, ParseFloatError}};

/// Type that can be read from standrard input.
pub trait FromStdin: Sized {
    type Error: std::error::Error;

    /// Reads the value from standrard input with optional radix.
    fn read_cin(cin: &mut Stdin, radix: Option<u32>) -> Result<Self, Self::Error>;
}

macro_rules! impl_from_cin_prim {
    ($($ty:ty),*) => {
        $(
            impl FromStdin for $ty {
                type Error = ParseIntError;

                // Parsing done manually in effort to avoid allocation, not sure if it's the best
                // way to go about this.
                fn read_cin(cin: &mut Stdin, radix: Option<u32>) -> Result<Self, Self::Error> {
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
                    <$ty>::from_str_radix(&s, radix.unwrap_or(10))
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
                    s.parse()
                }
            }
        )*
    };
}

impl_from_cin_prim!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
impl_from_cin_float!(f32, f64);

