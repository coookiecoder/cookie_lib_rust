use std::fmt;

pub struct Matrix<Type> {
    row: usize,
    col: usize,
    data: Vec<Vec<Type>>,
}

impl<Type> From<Vec<Vec<Type>>> for Matrix<Type> {
    fn from(data: Vec<Vec<Type>>) -> Matrix<Type> {
        let col = data[0].len();

        for row in &data {
            if row.len() != col {
                panic!("Matrix data must have equal row!");
            }
        }

        Matrix { row: data.len(), col: data[0].len(), data:data }
    }
}

impl<Type: fmt::Debug> fmt::Debug for Matrix<Type> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix ({}x{}):", self.row, self.col)?;
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

