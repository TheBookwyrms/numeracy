use crate::matrices::{Matrix as Matrix3, S1};
use crate::traits::Float;
use crate::vectors::vector::Vector;

impl<T:Float, const LEN:usize> Vector<T, LEN> {

    /// returns a null vector of length len
    pub fn zeroes() -> Vector<T, LEN> {
        let arr = vec![T::zero(); LEN];
        Vector { array: arr }
    }
}

impl<T:Clone, const LEN:usize> Vector<T, LEN> {
    /// creates a 1-dimensional matrix from a slice
    pub fn from_slice(slice:&[T]) -> Vector<T, LEN> {
        Vector {array: slice.to_vec()}
    }

    /// creates vector from a scalar value
    pub fn from_scalar(f:T) -> Vector<T, LEN> {
        Vector { array:vec![f] }
    }

    /// creates a vector from an array
    pub fn from_1darray(arr:[T;LEN]) -> Vector<T, LEN> {
        Vector { array:arr.to_vec() }
    }

    /// creates a vector from a vec
    pub fn from_vec(vec:Vec<T>) -> Vector<T, LEN> {
        Vector { array: vec }
    }

    pub fn from_matrix3<const M:usize>(mat:Matrix3<T, 1, S1<LEN>>) -> Vector<T, LEN> {
        Vector { array:mat.array }
    }
}