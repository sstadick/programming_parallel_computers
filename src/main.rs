/// Perform one step of min-plus matrix multiplication
/// n deontes the number of nodes
/// d contains the input matrix ex: (i, j) = d[n*i + j]
/// r contains the resul
fn step(r: &mut Vec<f64>, d: &Vec<f64>, n: usize) {}

fn main() {
    let n = 3;
    let d = vec![0.0, 8.0, 2.0, 1.0, 0.0, 9.0, 4.0, 5.0, 0.0];
    let mut r = Vec::with_capacity(n * n);
    step(&mut r, &d, n);
    for i in 0..n {
        for j in 0..n {
            print!("{} ", r[i * n + j]);
        }
        println!();
    }
}
