use saneput::input;

fn main() {
    print!("Your first name: ");
    let first = input!("{String}");

    print!("Your last name: ");
    let last = input!("{String}");

    println!("Hello, {first} {last}");
}
