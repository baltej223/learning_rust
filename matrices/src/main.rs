#![allow(dead_code)]
#![allow(unused_variables)]

struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn construct(data: Vec<Vec<i32>>) -> Matrix {
        let rows = data.len();

        let cols = data[0].len();
        for i in 1..data.len() {
            if data[i].len() != cols {
                panic!("all the columns should be of same number.");
            }
        }
        Matrix { rows, cols, data }
    }

    fn print_matrix(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.data[row][col]);
            }
            println!(" ");
        }
        println!(" ");
    }

    fn construct_zero_matrix(rows: usize, cols: usize) -> Matrix {
        let mut zero_vector = Vec::new();
        for row in 0..rows {
            let mut inner_zero_vector = Vec::new();
            for col in 0..cols {
                inner_zero_vector.push(0);
            }
            zero_vector.push(inner_zero_vector);
        }
        Matrix::construct(zero_vector)
    }

    fn transpose(&mut self) {
        let rows = self.rows;
        let cols = self.cols;
        let mut transposed_matrix: Matrix = Self::construct_zero_matrix(rows, cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                transposed_matrix.data[row][col] = self.data[col][row];
            }
        }
        self.data = transposed_matrix.data;
    }

    fn construct_identity_matrix(n: usize) -> Matrix {
        let mut zero_matrix = Matrix::construct_zero_matrix(n, n);
        for row in 0..n {
            for col in 0..n {
                if row == col {
                    zero_matrix.data[row][col] = 1;
                }
            }
        }
        zero_matrix
    }
}
fn main() {
    let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut matrix = Matrix::construct(vec);
    matrix.print_matrix();
    matrix.transpose();
    matrix.print_matrix();
    Matrix::construct_identity_matrix(12).print_matrix();
}
