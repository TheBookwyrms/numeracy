use crate::general_math::multiplications::sum_of_multiplications;
use crate::{traits::Float, vectors::vector::Vector};
use crate::traits::Numerical;
use crate::enums::MatrixError;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul};

impl<T:Numerical> Add<Vector<T>> for Vector<T> {
    type Output = Result<Vector<T>, MatrixError>;

    /// add two matrices together element-wise
    fn add(self, other: Self) -> Result<Vector<T>, MatrixError> {
        if self.num_items() != other.num_items() {
            Err(MatrixError::InvalidItemNumbers(vec![self.num_items(), other.num_items()]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other[idx]);
            
            Ok(Vector {array:v})
        }
    }
}



impl<T:Numerical> Add<T> for Vector<T> {
    type Output = Vector<T>;
    /// adds an element to all items of a matrix
    fn add(self, other: T) -> Vector<T> {

        let mut v = self.array.clone();
        v.iter_mut().for_each(|val| *val = *val+other);
        
        Vector { array:v }
    }
}

impl<T:Numerical> Sub<Vector<T>> for Vector<T> {
    type Output = Result<Self, MatrixError>;

    /// subtracts two matrices element-wise
    fn sub(self, other: Self) -> Result<Self, MatrixError> {
        if self.num_items() != other.num_items() {
            Err(MatrixError::InvalidItemNumbers(vec![self.num_items(), other.num_items()]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-other[idx]);

            Ok(Vector {array:v})
        }
    }
}


impl<T:Numerical> Sub<T> for Vector<T> {
    type Output = Vector<T>;
    /// subtracts an element to all items of a matrix
    fn sub(self, other: T) -> Vector<T> {

        let mut v = self.array.clone();
        v.iter_mut().for_each(|val| *val = *val-other);
        
        Vector { array:v }
    }
}

impl<T:Numerical + Neg<Output = T>> Neg for Vector<T> {
    type Output = Vector<T>;
    /// returns the vector where every element is its negative self
    fn neg(self) -> Vector<T> {

        let mut v = self.array.clone();
        v.iter_mut().for_each(|val| *val = -*val);
        
        Vector { array:v }
    }
}

impl <T:Numerical> Mul<T> for Vector<T> {
    type Output = Vector<T>;
    /// returns the vector where every element is multiplied by the other
    fn mul(self, rhs: T) -> Self::Output {
        let mut v = self.array.clone();
        v.iter_mut().for_each(|val| *val = rhs * *val);
        
        Vector { array:v }
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
        } else {
            Ok(sum_of_multiplications(&self.array, &other.array))
        }
    }

    pub fn cross_product(&self, other:&Vector<T>) -> Result<Vector<T>, MatrixError> {
        if !self.is_vec3() || !other.is_vec3() {
            Err(MatrixError::InvalidItemNumbers(vec![self.num_items(), other.num_items()]))
        } else {
            let (ax, ay, az) = (self[0], self[1], self[2]);
            let (bx, by, bz) = (other[0], other[1], other[2]);

            let i = ay*bz - az*by;
            let j = az*bx - ax*bz;
            let k = ax*by - ay*bx;

            Ok(Vector { array: vec![i, j, k] })
        }
    }

    /// multiplies every element of an n-dimensional matrix by a scalar value
    pub fn multiply_by_constant(&self, scalar:T) -> Vector<T> {
        let mut narr = self.array.clone();
        (0..narr.len()).for_each(|i| narr[i] *= scalar.clone());
        Vector {array:narr}
    }
}


impl<T:Float> Vector<T> {
    pub fn magnitude(&self) -> T {
        T::sqrt(sum_of_multiplications(&self.array, &self.array))
    }

    pub fn project_onto(&self, other:&Vector<T>) -> Result<Vector<T>, MatrixError> {
        Ok(other.clone().multiply_by_constant(self.dot(&other)?/self.dot(&self)?))
    }

    pub fn normalise(&self) -> Result<Vector<T>, MatrixError> {
        if self.array.iter().all(|a| *a==T::zero()) {
            Err(MatrixError::NullVector)
        } else {
            Ok(self.multiply_by_constant(T::one()/self.magnitude()))
        }
    }
}