use crate::matrix::Matrix;

use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use std::ops::Deref;

pub struct Vector2D<Type> {
    data: Matrix<Type>,
}

impl<Type> Deref for Vector2D<Type> {
    type Target = Matrix<Type>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Type: Clone> From<Matrix<Type>> for Vector2D<Type> {
    fn from(data: Matrix<Type>) -> Vector2D<Type> {
        if data.get_row() != 2 {
            panic!("Invalid vector row dimensions.");
        } else if data.get_col() != 1 {
            panic!("Invalid vector column dimensions.");
        }
        
        Vector2D { data: data }
    }
}

impl<Type: Debug> Debug for Vector2D<Type> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.data.fmt(f) // Delegate to Matrix's Debug implementation
    }
}

impl<Type: Display> Display for Vector2D<Type> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.data.fmt(f) // Delegate to Matrix's Display implementation
    }
}

impl<Type: Clone> Vector2D<Type> {
    pub fn new(x: Type, y: Type) -> Vector2D<Type> {
        let buffer: Vec<Vec<Type>> = vec![vec![x], vec![y]];
        
        Vector2D { data: Matrix::from(&buffer) }
    }

    pub fn get_x(&self) -> Type {
        self.data.get_data_item(0, 0)
    }

    pub fn get_y(&self) -> Type {
        self.data.get_data_item(1, 0)
    }
}

