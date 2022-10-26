use saneput::input;

fn print_mat(mat: &Vec<Vec<f32>>) {
    for row in mat.iter() {
        for e in row.iter() {
            print!("{e} ");
        }
        println!("");
    }
}

fn iter_matrix(
    mat: &Vec<Vec<f32>>,
) -> impl Iterator<Item = impl Iterator<Item = f32> + '_> + Clone + '_ {
    mat.iter().map(|r| r.iter().copied())
}

fn select_minor<'a>(
    mat: impl Iterator<Item = impl Iterator<Item = f32> + 'a> + Clone + 'a,
    n: usize,
    m: usize,
) -> impl Iterator<Item = impl Iterator<Item = f32> + 'a> + Clone + 'a {
    mat.enumerate()
        .filter(move |(i, _)| i + 1 != n)
        .map(move |(_, r)| {
            r.enumerate()
                .filter(move |(i, _)| i + 1 != m)
                .map(|(_, v)| v)
        })
}

fn compute_determinant(t: impl Iterator<Item = impl Iterator<Item = f32>>) -> f32 {
    let minor = t.map(|t| t.collect::<Vec<_>>()).collect::<Vec<_>>();

    if minor.len() == 1 && minor[0].len() == 1 {
        minor[0][0]
    } else {
        let mut sum = 0.;
        for (i, mut e) in minor[0].iter().copied().enumerate() {
            let sub = select_minor(iter_matrix(&minor), 1, i + 1);
            e *= if i % 2 == 0 { 1. } else { -1. };

            let subdet = compute_determinant(sub);
            sum += e * subdet;
        }
        sum
    }
}

fn main() {
    println!(
        "This example performs Laplace(cofactor) expansion to compute the determinant of a matrix."
    );
    print!("Enter matrix size n: ");
    let n = input!("{usize}");

    let mut mat: Vec<Vec<f32>> = vec![vec![0.; n]; n];

    print!("Enter matrix elements(row major): ");
    for i in 0..n * n {
        let e = input!("{f32}");
        mat[i / n][i % n] = e;
    }
    print!("\n");

    let det = compute_determinant(iter_matrix(&mat));
    println!("The determinant is: {det}");

    println!("Matrix is: ");
    print_mat(&mat);
}
