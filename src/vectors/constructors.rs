use crate::traits::{IntoDataType, Float};
use crate::enums::MatrixDataTypes;
use crate::vectors::vector::Vector;

impl<T:Float> Vector<T> {

    /// returns a null vector of length len
    pub fn null(len:usize) -> Vector<T> {
        let arr = vec![T::zero(); len];
        Vector { array: arr, dtype: T::as_dtype() }
    }
}

impl<T:IntoDataType + Clone> Vector<T> {

    /// returns an empty vector
    pub fn new_empty() -> Vector<T> {
        Vector { array: vec![], dtype:MatrixDataTypes::EMPTY }
    }

    /// creates vector from a scalar value
    pub fn from_scalar(f:T) -> Vector<T> {
        let dtype = T::as_dtype();
        Vector { array:vec![f], dtype:dtype }
    }

    /// creates a vector from an array
    pub fn from_1darray<const M:usize>(arr:[T;M]) -> Vector<T> {
        let dtype = T::as_dtype();
        Vector { array:arr.to_vec(), dtype }
    }

    /// creates a vector from a vec
    pub fn from_vec(vec:Vec<T>) -> Vector<T> {
        let dtype = T::as_dtype();
        Vector { array: vec, dtype }
    }
}