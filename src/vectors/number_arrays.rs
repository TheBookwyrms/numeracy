use crate::general_math::multiplications::sum_of_multiplications;
use crate::{traits::Float, vectors::vector::Vector};
use crate::traits::Numerical;
use crate::enums::MatrixError;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul};

impl<T:Numerical, const LEN:usize> Add<Vector<T, LEN>> for Vector<T, LEN> {
    type Output = Self;

    /// add two matrices together element-wise
    fn add(self, other: Self) -> Self {
        let mut v = self.array;
        v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other[idx]);
        
        Vector {array:v}
    }
}



impl<T:Numerical, const LEN:usize> Add<T> for Vector<T, LEN> {
    type Output = Self;
    /// adds an element to all items of a matrix
    fn add(self, other: T) -> Self {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = *val+other);
        
        Vector { array:v }
    }
}

impl<T:Numerical, const LEN:usize> Sub<Vector<T, LEN>> for Vector<T, LEN> {
    type Output = Self;

    /// subtracts two matrices element-wise
    fn sub(self, other: Self) -> Self {
        let mut v = self.array;
        v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-other[idx]);

        Vector {array:v}
    }
}


impl<T:Numerical, const LEN:usize> Sub<T> for Vector<T, LEN> {
    type Output = Self;
    /// subtracts an element to all items of a matrix
    fn sub(self, other: T) -> Self {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = *val-other);
        
        Vector { array:v }
    }
}

impl<T:Numerical + Neg<Output = T>, const LEN:usize> Neg for Vector<T, LEN> {
    type Output = Self;
    /// returns the vector where every element is its negative self
    fn neg(self) -> Self {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = -*val);
        
        Vector { array:v }
    }
}

impl <T:Numerical, const LEN:usize> Mul<T> for Vector<T, LEN> {
    type Output = Self;
    /// returns the vector where every element is multiplied by the other
    fn mul(self, rhs: T) -> Self::Output {
        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = rhs * *val);
        
        Vector { array:v }
    }
}


impl<T:Numerical, const LEN:usize> AddAssign for Vector<T, LEN> {
    fn add_assign(&mut self, rhs: Self) {
        self.array.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+rhs.array[idx]);
    }
}
///i wonder when you will find this. 23:28 dec 27 2025
impl<T:Numerical, const LEN:usize> SubAssign for Vector<T, LEN> {
    fn sub_assign(&mut self, rhs: Self) {
        self.array.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-rhs.array[idx]);
    }
}

impl<T:Numerical, const LEN:usize> Vector<T, LEN> {
    
    /// performs the dot product of two vectors (1D matrices) 
    pub fn dot(&self, other:&Self) -> T {
        sum_of_multiplications(&self.array, &other.array)
    }

    /// multiplies every element of an n-dimensional matrix by a scalar value
    pub fn multiply_by_constant(self, scalar:T) -> Self {
        let mut narr = self.array;
        (0..narr.len()).for_each(|i| narr[i] *= scalar);
        Vector {array:narr}
    }
}

impl<T:Numerical> Vector<T, 3> {
    pub fn cross_product(&self, other:&Self) -> Self {
        let (ax, ay, az) = (self[0], self[1], self[2]);
        let (bx, by, bz) = (other[0], other[1], other[2]);

        let i = ay*bz - az*by;
        let j = az*bx - ax*bz;
        let k = ax*by - ay*bx;

        Vector { array: vec![i, j, k] }
    }
}


impl<T:Float, const LEN:usize> Vector<T, LEN> {
    pub fn magnitude(&self) -> T {
        T::sqrt(sum_of_multiplications(&self.array, &self.array))
    }

    pub fn project_onto(&self, other:Self) -> Self {
        let projection_factor = self.dot(&other)/self.dot(&self);
        other.multiply_by_constant(projection_factor)
    }

    pub fn normalise(self) -> Result<Self, MatrixError> {
        if self.array.iter().all(|a| *a==T::zero()) {
            Err(MatrixError::NullVector)
        } else {
            let magnitude = self.magnitude();
            Ok(self.multiply_by_constant(T::one()/magnitude))
        }
    }
}