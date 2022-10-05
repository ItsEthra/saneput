use std::io;
use saeput::*;

fn main() {
    let mut cin = io::stdin();

    let v1 = i32::read_cin(&mut cin, Some(ExpectedRadix::Dec))
        .unwrap();
    dbg!(v1);

    let v2 = u32::read_cin(&mut cin, Some(ExpectedRadix::Dec))
        .unwrap();
    dbg!(v2);
}
