use std::fmt;

struct MatrixFloat {
    row: usize,
    col: usize,
    data: Vec<Vec<f64>>
}

struct MatrixInt {
    row: usize,
    col: usize,
    data: Vec<Vec<u64>>
}

pub struct Matrix {
    row: usize,
    col: usize,
    matrix_int: MatrixInt,
    matrix_float: MatrixFloat
}

impl From<Vec<Vec<u64>>> for Matrix {
    fn from(data: Vec<Vec<u64>>) -> Matrix {
        let col = data[0].len();

        for row in data.clone() {
            if row.len() != col {
                panic!("Matrix data must have equal row!");
            }
        }
        
        let matrix_float: MatrixFloat = MatrixFloat { row: data.len(), col: data[0].len(), data: vector_2d_int_to_float(data.clone()) };
        let matrix_int: MatrixInt = MatrixInt { row: data.len(), col: data[0].len(), data:data.clone() };

        Matrix { row: data.len(), col: data[0].len(), matrix_int, matrix_float }
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(data: Vec<Vec<f64>>) -> Matrix {
        let col = data[0].len();
        
        for row in data.clone() {
            if row.len() != col {
                panic!("Matrix data must have equal row!");
            }
        }
        
        let matrix_int: MatrixInt = MatrixInt { row: data.len(), col: data[0].len(), data:vector_2d_float_to_int(data.clone()) };
        let matrix_float: MatrixFloat = MatrixFloat { row: data.len(), col: data[0].len(), data:data.clone() };

        Matrix { row: data.len(), col: data[0].len(), matrix_int, matrix_float }
    }
}

impl fmt::Debug for MatrixInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MatrixInt ({}x{}):", self.row, self.col)?;
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl fmt::Debug for MatrixFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MatrixFloat ({}x{}):", self.row, self.col)?;
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix ({}x{}):", self.row, self.col)?;
        writeln!(f, "MatrixInt:\n{:?}", self.matrix_int)?;
        writeln!(f, "MatrixFloat:\n{:?}", self.matrix_float)
    }
}

fn vector_2d_int_to_float(data:Vec<Vec<u64>>) -> Vec<Vec<f64>> {
    let mut new_2d_vector:Vec<Vec<f64>> = Vec::new();
    let mut new_1d_vector:Vec<f64> = Vec::new();

    if data.is_empty() {
        panic!("Matrix data must have at least one row.");
    }

    if data[0].is_empty() {
        panic!("Matrix data must have at least one column.");
    }

    for row in data.clone() {
        for item in row.clone() {
            new_1d_vector.push(item as f64);
        }
        new_2d_vector.push(new_1d_vector.clone());
        new_1d_vector.clear();
    }

    new_2d_vector
}

fn vector_2d_float_to_int(data:Vec<Vec<f64>>) -> Vec<Vec<u64>> {
    let mut new_2d_vector:Vec<Vec<u64>> = Vec::new();
    let mut new_1d_vector:Vec<u64> = Vec::new();

    if data.is_empty() {
        panic!("Matrix data must have at least one row.");
    }

    if data[0].is_empty() {
        panic!("Matrix data must have at least one column.");
    }

    for row in data.clone() {
        for item in row.clone() {
            new_1d_vector.push(item as u64);
        }
        new_2d_vector.push(new_1d_vector.clone());
        new_1d_vector.clear();
    }

    new_2d_vector
}
