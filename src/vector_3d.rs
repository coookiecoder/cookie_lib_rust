use crate::matrix::Matrix;

use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use std::ops::Deref;

pub struct Vector3D<Type> {
    data: Matrix<Type>,
}

impl<Type> Deref for Vector3D<Type> {
    type Target = Matrix<Type>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Type: Clone> From<Matrix<Type>> for Vector3D<Type> {
    fn from(data: Matrix<Type>) -> Vector3D<Type> {
        if data.get_row() != 3 {
            panic!("Invalid vector row dimensions.");
        } else if data.get_col() != 1 {
            panic!("Invalid vector column dimensions.");
        }
        
        Vector3D { data:data }
    }
}

impl<Type: Debug> Debug for Vector3D<Type> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.data.fmt(f) // Delegate to Matrix's Debug implementation
    }
}

impl<Type: Display> Display for Vector3D<Type> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.data.fmt(f) // Delegate to Matrix's Display implementation
    }
}

impl<Type: Clone> Vector3D<Type> {
    pub fn new(x: Type, y: Type, z: Type) -> Vector3D<Type> {
        let buffer: Vec<Vec<Type>> = vec![vec![x], vec![y], vec![z]];

        Vector3D { data: Matrix::from(&buffer) }
    }

    pub fn get_x(&self) -> Type {
        self.data.get_data_item(0, 0)
    }

    pub fn get_y(&self) -> Type {
        self.data.get_data_item(1, 0)
    }

    pub fn get_z(&self) -> Type {
        self.data.get_data_item(2, 0)
    }
}