# Saneput
Rust sane input reading library

# Usage
```rust
use saneput::input;
// By default type `input` parses is `i32`.
let value = input!("{}");
// > -15
// value = -15

// You can also specify the radix input string should be in. Though no prefixes are allowed.
// `b` - binary, `o` - octal, `d` - decimal(defualt), `x` - hexidecimal.
// `{:x}` - is called a group.
// Accepted types are all integers types, `f32` and `f64`.
let thing: u64 = input!("{u64:x}");
// > ff
// thing = 255

// Returns a tuple. You can enter number on the same line separated by space/tab
// > 1 2
// Or you can enter them one by one with each on a separate line.
// > 1
// > 2
let (a, b): (i32, u32) = input!("{i32}{u32}");
/*                                   ^^
                                     ||
                            Notice lack of space.
All groups must follow each other with no characters in between. */

// For more comprehensive example take a look at `examples/grid.rs`
```


# Installation
```toml
[dependencies]
saneput = "0.1"

# For development version
saneput = { git = "https://github.com/ItsEthra/saneput" }
```
