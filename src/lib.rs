pub mod matrix;
pub mod vector;

#[cfg(test)]
mod test_matrix {
    #[test]
    fn matrix_creation() {
        use crate::matrix::Matrix;
        
        let custom_data: Vec<Vec<u64>> = vec![vec![0; 3]; 3];
        let matrix:Matrix<u64> = Matrix::from(&custom_data);
        println!("{:?}", matrix);
        println!("{}", matrix);

        let custom_data: Vec<Vec<f64>> = vec![vec![0.0; 3]; 3];
        let matrix:Matrix<f64> = Matrix::from(&custom_data);
        println!("{:?}", matrix);
        println!("{}", matrix);

        let custom_data: Vec<Vec<f64>> = vec![vec![0.0; 3]; 2];
        let matrix:Matrix<f64> = Matrix::from(&custom_data);
        println!("{:?}", matrix);
        println!("{}", matrix);
        
        let custom_data: Vec<Vec<u64>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let matrix:Matrix<u64> = Matrix::from(&custom_data);
        println!("{:?}", matrix);
        println!("{}", matrix);
        
        assert_eq!(matrix.get_data_item(0,0), 1);
        assert_eq!(matrix.get_data_item(0,1), 2);
        assert_eq!(matrix.get_data_item(0,2), 3);
        assert_eq!(matrix.get_data_item(1,0), 4);
        assert_eq!(matrix.get_data_item(1,1), 5);
        assert_eq!(matrix.get_data_item(1,2), 6);
        assert_eq!(matrix.get_data_item(2,0), 7);
        assert_eq!(matrix.get_data_item(2,1), 8);
        assert_eq!(matrix.get_data_item(2,2), 9);
    }
    
    #[test]
    fn matrix_add() {
        use crate::matrix::Matrix;

        let custom_data: Vec<Vec<u64>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        
        let matrix:Matrix<u64> = Matrix::from(&custom_data);
        let result:Matrix<u64> = Matrix::from(&custom_data);
        
        println!("before add :\n");
        println!("{}", matrix);
        println!("{}", result);
        
        let result:Matrix<u64> = matrix.add(&matrix);
        
        println!("after add :\n{}", result);
        println!("matrix : {}", matrix);
        println!("result : {}", result);
    }
}

#[cfg(test)]
mod test_vector {
    #[test]
    fn vector_2d_creation() {
        use crate::vector::Vector2D;

        let vector:Vector2D<f64> = Vector2D::new(1.0, 2.0);
        println!("{:?}", vector);
        println!("{}", vector);

        let vector:Vector2D<u64> = Vector2D::new(1, 2);
        println!("{:?}", vector);
        println!("{}", vector);

        assert_eq!(vector.get_x(), 1);
        assert_eq!(vector.get_y(), 2);
    }

    #[test]
    fn vector_3d_creation() {
        use crate::vector::Vector3D;

        let vector:Vector3D<f64> = Vector3D::new(1.0, 2.0, 3.0);
        println!("{:?}", vector);
        println!("{}", vector);

        let vector:Vector3D<u64> = Vector3D::new(1, 2, 3);
        println!("{:?}", vector);
        println!("{}", vector);
        
        assert_eq!(vector.get_x(), 1);
        assert_eq!(vector.get_y(), 2);
        assert_eq!(vector.get_z(), 3);
    }
}
