use crate::{traits::Float, vectors::vector::Vector};
use crate::traits::Numerical;
use crate::enums::MatrixError;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl<T:Numerical> Add for Vector<T> {
    type Output = Result<Vector<T>, MatrixError>;

    /// add two matrices together element-wise
    fn add(self, other: Self) -> Result<Vector<T>, MatrixError> {
        if self.num_items() != other.num_items() {
            Err(MatrixError::InvalidItemNumbers(vec![self.num_items(), other.num_items()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other[idx]);
            
            Ok(Vector {array:v, dtype:self.dtype})
        }
    }
}

impl<T:Numerical> Sub for Vector<T> {
    type Output = Result<Self, MatrixError>;

    /// subtracts two matrices element-wise
    fn sub(self, other: Self) -> Result<Self, MatrixError> {
        if self.num_items() != other.num_items() {
            Err(MatrixError::InvalidItemNumbers(vec![self.num_items(), other.num_items()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-other[idx]);

            Ok(Vector {array:v, dtype:self.dtype})
        }
    }
}

impl<T:Numerical> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.array.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+rhs.array[idx]);
    }
}
///i wonder when you will find this. 23:28 dec 27 2025
impl<T:Numerical> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.array.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-rhs.array[idx]);
    }
}

impl<T:Numerical> Vector<T> {
    
    /// performs the dot product of two vectors (1D matrices) 
    pub fn dot(&self, other:&Self) -> Result<T, MatrixError> {
        if self.num_items() != other.num_items() {
            Err(MatrixError::InvalidItemNumbers(vec![self.num_items(), other.num_items()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {

            let mut sums = self.array.clone();

            sums.iter_mut().enumerate().for_each(|(idx, val)| *val = *val*other[idx]);

            Ok(sums.into_iter().sum())
        }
    }

    pub fn cross_product(&self, other:&Vector<T>) -> Result<Vector<T>, MatrixError> {
        if !(self.num_items()==3) || !(other.num_items()==3) {
            Err(MatrixError::InvalidItemNumbers(vec![self.num_items(), other.num_items()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {

            let (ax, ay, az) = (self[0], self[1], self[2]);
            let (bx, by, bz) = (other[0], other[1], other[2]);

            let i = ay*bz - az*by;
            let j = az*bx - ax*bz;
            let k = ax*by - ay*bx;

            Ok(Vector { array: vec![i, j, k], dtype: self.dtype })
        }
    }

    /// multiplies every element of an n-dimensional matrix by a scalar value
    pub fn multiply_by_constant(&self, scalar:T) -> Vector<T> {
        let mut narr = self.array.clone();
        (0..self.array.len()).for_each(|i| narr[i] *= scalar.clone());
        Vector {array:narr, dtype:self.dtype}
    }
}


impl<T:Float> Vector<T> {
    pub fn magnitude(&self) -> Result<T, MatrixError> {
        Ok(T::sqrt(self.dot(&self)?))
    }

    pub fn project_onto(&self, other:&Vector<T>) -> Result<Vector<T>, MatrixError> {
        Ok(other.multiply_by_constant(self.dot(&other)?/self.dot(&self)?))
    }

    pub fn normalise(&self) -> Result<Vector<T>, MatrixError> {
        Ok(self.clone().multiply_by_constant(T::one()/self.magnitude()?))
    }
}