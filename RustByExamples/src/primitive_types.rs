use std::fmt::Display;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let matrix = (matrix.0, matrix.1, matrix.2, matrix.3);
    let (a, b, c, d) = matrix;

    Matrix(a,c,b,d)
}

pub fn tuples() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix: \n{}", matrix);
    println!("Transpose: \n{}",  transpose(matrix));
}