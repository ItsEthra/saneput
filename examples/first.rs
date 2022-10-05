use std::io;
use saeput::*;

fn main() {
    let mut cin = io::stdin();

    let v = input!("{i32}{}{u8:x}{:b}");
    let v = input!("{i32}");
    dbg!(v);

    let v1 = i32::read_cin(&mut cin, Some(ExpectedRadix::Dec))
        .unwrap();
    dbg!(v1);

    let v2 = u32::read_cin(&mut cin, Some(ExpectedRadix::Dec))
        .unwrap();
    dbg!(v2);
}
