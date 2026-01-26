#![allow(dead_code)]
#![allow(unused_variables)]

struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

fn construct(data: Vec<Vec<i32>>) -> Matrix<i32> {
    let rows = data.len();

    let cols = data[0].len();
    for i in 1..data.len() {
        if data[i].len() != cols {
            panic!("all the columns should be of same number.");
        }
    }
    Matrix { rows, cols, data }
}

fn print_matrix(matrix: &Matrix<i32>) {
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            print!("{} ", matrix.data[row][col]);
        }
        println!(" ");
    }
    println!(" ");
}

fn construct_zero_matrix(rows: usize, cols: usize) -> Matrix<i32> {
    let mut zero_vector = Vec::new();
    for row in 0..rows {
        let mut inner_zero_vector = Vec::new();
        for col in 0..cols {
            inner_zero_vector.push(0);
        }
        zero_vector.push(inner_zero_vector);
    }
    construct(zero_vector)
}

fn transpose(matrix: Matrix<i32>) -> Matrix<i32> {
    let rows = matrix.rows;
    let cols = matrix.cols;
    let mut transposed_matrix: Matrix<i32> = construct_zero_matrix(rows, cols);
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            transposed_matrix.data[row][col] = matrix.data[col][row];
        }
    }
    transposed_matrix
}

fn construct_identity_matrix(rows: usize, cols: usize) -> Matrix<i32> {
    let mut zero_matrix = construct_zero_matrix(rows, cols);
    for row in 0..rows {
        for col in 0..cols {
            if row == col {
                zero_matrix.data[row][col] = 1;
            }
        }
    }
    zero_matrix
}

fn main() {
    let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let matrix = construct(vec);
    print_matrix(&matrix);
    let transposed_mat = transpose(matrix);
    print_matrix(&transposed_mat);
    print_matrix(&construct_identity_matrix(12, 5));
}
