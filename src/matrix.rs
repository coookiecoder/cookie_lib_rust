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
        if self.col == 1 {
            writeln!(f, "Vector ({:?}x{:?}):", self.row, self.col)?;
        } else {
            writeln!(f, "Matrix ({:?}x{:?}):", self.row, self.col)?;
        }
        
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        
        Ok(())
    }
}

impl <Type: fmt::Display> fmt::Display for Matrix<Type> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.col == 1 {
            writeln!(f, "Vector ({}x{}):", self.row, self.col)?;
        } else {
            writeln!(f, "Matrix ({}x{}):", self.row, self.col)?;
        }
        
        for row in &self.data {
            write!(f, "[", )?;
            for item in row {
                write!(f, " {} ", item)?;
            }
            writeln!(f, "]")?;
        }
        
        Ok(())
    }
}

impl<Type: Clone> Matrix<Type> {
    pub fn new(row: usize, col: usize, data: Vec<Vec<Type>>) -> Matrix<Type> {
        Matrix { row: row, col: col, data: data }
    }
    
    pub fn get_data(&self) -> Vec<Vec<Type>> {
        self.data.clone()
    }
    
    pub fn get_data_row(&self, row: usize) -> Vec<Type> {
        if row >= self.row {
            panic!("Row index out of bounds!");
        }
        
        self.data[row].clone()
    }
    
    pub fn get_data_item(&self, row: usize, col: usize) -> Type {
        if row >= self.row {
            panic!("Row index out of bounds!");
        } else if col >= self.col {
            panic!("Column index out of bounds!");
        }
        
        self.data[row][col].clone()
    }
    
    pub fn is_square(&self) -> bool {
        self.row == self.col
    }
    
    pub fn is_vector(&self) -> bool {
        self.col == 1
    }
}