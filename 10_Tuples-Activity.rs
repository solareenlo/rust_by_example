use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;
    return Matrix(a, c, b, d);
}

fn main() {
    let matrix = Matrix(1.1, 2.2, 3.3, 4.4);
    println!("Matrix:");
    println!("{:?}", matrix);
    println!("{:}", matrix);
    println!();

    println!("Trnspose:");
    println!("{}", transpose(matrix));
}
