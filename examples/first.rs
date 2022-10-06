use saneput::*;

fn main() {
    let n: i32 = input!("{}");

    for i in 0..n {
        let (a, b) = input!("{}{f32}");
        dbg!(i, (a, b));
    }
}
