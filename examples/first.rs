use std::{io::{self, Read, Stdin}, slice};

fn read_i32(cin: &mut Stdin) -> io::Result<i32> {
    let (mut b, mut v) = (0, [0u8; 10]);

    let mut i = 0u32;
    loop {
        cin.read(slice::from_mut(&mut b))?;
        if b == b' ' || b == b'\n' {
            break;
        } else if !(b as char).is_numeric() {
            todo!("Return an error");
        }
        v[i as usize] = b - b'0';
        i += 1;
    }


    let out = v.iter()
        .take(i as usize)
        .fold((i - 1, 0i32), |(power, value), b| {

            (power.saturating_sub(1), value + (*b as i32) * 10i32.pow(power))
        });

    Ok(out.1)
}

fn main() -> io::Result<()> {
    let mut cin = io::stdin();
    dbg!(read_i32(&mut cin)?);
    dbg!(read_i32(&mut cin)?);

    Ok(())
}
