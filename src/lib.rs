#![doc = include_str!("../README.md")]

mod fromcin;
pub use fromcin::*;

mod error;
pub use error::*;

pub use saneput_proc::input;
