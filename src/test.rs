mod matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn matrix_creation_float() {
        use crate::matrix::Matrix;
        
        let custom_data: Vec<Vec<f64>> = vec![vec![0.; 3]; 3];
        let matrix:Matrix = Matrix::from(custom_data);

        println!("{:?}", matrix);
    }
    #[test]
    fn matrix_creation_int() {
        use crate::matrix::Matrix;
        
        let custom_data: Vec<Vec<u64>> = vec![vec![0; 3]; 3];
        let matrix:Matrix = Matrix::from(custom_data);

        println!("{:?}", matrix);
    }
}
