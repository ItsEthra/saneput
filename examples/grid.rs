use saneput::*;

fn main() {
    // Create grid with `n` rows and `m` columns.
    print!("Enter size of the grid(n m): ");
    let (n, m) = input!("{usize}{usize}");

    let mut g: Vec<Vec<f32>> = vec![vec![0.; m]; n];

    print!("Number of replacements: ");
    let count = input!("{usize}");

    for i in 1..=count {
        print!("Replacement {i}: ");
        let (x, y, val) = input!("{usize}{usize}{f32}");

        g[y][x] = val;
    }

    print!("\n");
    for y in 0..m {
        for x in 0..n {
            print!("{:4} ", g[y][x]);
        }
        println!("");
    }
}
