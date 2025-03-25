mod matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn matrix_creation() {
        use crate::matrix::Matrix;
        
        let custom_data: Vec<Vec<u64>> = vec![vec![0; 3]; 3];
        let matrix:Matrix<u64> = Matrix::from(custom_data);
        println!("{:?}", matrix);

        let custom_data: Vec<Vec<f64>> = vec![vec![0.0; 3]; 3];
        let matrix:Matrix<f64> = Matrix::from(custom_data);
        println!("{:?}", matrix);
    }
}
