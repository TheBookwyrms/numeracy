use crate::matrices::Matrix;
use crate::traits::Float;
use crate::enums::MatrixError;
use crate::vectors::vector::Vector;

impl<T:Float> Vector<T> {

    /// returns a null vector of length len
    pub fn null(len:usize) -> Vector<T> {
        let arr = vec![T::zero(); len];
        Vector { array: arr }
    }
}

impl<T:Clone> Vector<T> {
    /// creates a 1-dimensional matrix from a slice
    pub fn from_slice(slice:&[T]) -> Vector<T> {
        Vector {array: slice.to_vec()}
    }

    /// creates vector from a scalar value
    pub fn from_scalar(f:T) -> Vector<T> {
        Vector { array:vec![f] }
    }

    /// creates a vector from an array
    pub fn from_1darray<const M:usize>(arr:[T;M]) -> Vector<T> {
        Vector { array:arr.to_vec() }
    }

    /// creates a vector from a vec
    pub fn from_vec(vec:Vec<T>) -> Vector<T> {
        Vector { array: vec }
    }

    pub fn from_matrix(mat:Matrix<T>) -> Result<Vector<T>, MatrixError> {
        let matrix = mat.squeeze_axes();
        if matrix.ndims() == 1 {
            Ok(Vector { array:matrix.array })
        } else {
            Err(MatrixError::NotAVector)
        }
    }
}