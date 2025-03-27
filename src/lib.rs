pub mod matrix;
pub mod vector;

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
        
        let custom_data: Vec<Vec<u64>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let matrix:Matrix<u64> = Matrix::from(custom_data);
        println!("{:?}", matrix);
        
        assert_eq!(matrix.get_data_item(0,0), 1);
        assert_eq!(matrix.get_data_item(0,1), 2);
        assert_eq!(matrix.get_data_item(0,2), 3);
        assert_eq!(matrix.get_data_item(1,0), 4);
        assert_eq!(matrix.get_data_item(1,1), 5);
        assert_eq!(matrix.get_data_item(1,2), 6);
        assert_eq!(matrix.get_data_item(2,0), 7);
        assert_eq!(matrix.get_data_item(2,1), 8);
        assert_eq!(matrix.get_data_item(2,2), 9);
        
        println!("{}", matrix);
    }
}
