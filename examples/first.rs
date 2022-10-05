use std::{io::{self, Read, Stdin}, slice};

fn read_i32(cin: &mut Stdin) -> io::Result<i32> {
    let (mut b, mut v, mut neg) = (0, 0, None);

    loop {
        cin.read(slice::from_mut(&mut b))?;

        if b == b' ' || b == b'\n' {
            break;
        } else if b == b'-' && neg.is_none() {
            neg = Some(true);
            continue;
        } else if !(b as char).is_numeric() {
            todo!("Return an error");
        }

        v *= 10;
        v += (b - b'0') as i32;

        if v >= i32::MAX || v < i32::MIN {
            todo!("Return an error")
        }
    }


    // There is no way for `neg` to be `Some(false)` so its fine.
    Ok(v * neg.map(|_| -1).unwrap_or(1))
}

fn main() -> io::Result<()> {
    let mut cin = io::stdin();
    dbg!(read_i32(&mut cin)?);
    dbg!(read_i32(&mut cin)?);

    Ok(())
}
