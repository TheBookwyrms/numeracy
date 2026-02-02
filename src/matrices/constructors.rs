use crate::matrices::matrix::Matrix;
use crate::traits::Numerical;
use crate::enums::MatrixDataTypes;
use crate::enums::MatrixError;
use crate::vectors::Vector;

impl<T:Numerical> Matrix<T> {

    /// creates an empty matrix of given shape
    pub fn new_empty(shape:Vec<usize>) -> Matrix<T> {
        Matrix { shape, array: vec![], dtype:MatrixDataTypes::EMPTY }
    }

    /// creates matrix from a scalar value
    pub fn from_scalar(f:T) -> Matrix<T> {
        let dtype = T::as_dtype();
        Matrix {shape:vec![1], array:vec![f], dtype:dtype}
    }

    /// creates a 1-dimensional matrix from an array
    pub fn from_1darray<const M:usize>(arr:[T;M]) -> Matrix<T> {
        let dtype = T::as_dtype();
        Matrix {shape:vec![arr.len()], array:arr.to_vec(), dtype}
    }

    pub fn from_vector(vector:Vector<T>) -> Matrix<T> {
        Matrix { shape: vec![vector.num_items()], array: vector.array, dtype: vector.dtype }
    }

    /// creates a 1-dimensional matrix from a vec
    pub fn from_vec(vec:Vec<T>) -> Matrix<T> {
        let dtype = T::as_dtype();
        Matrix {shape:vec![vec.len()], array: vec, dtype}
    }

    /// creates a 2-dimensional matrix from a vec of vecs
    pub fn from_vec_of_vec(vec:Vec<Vec<T>>) -> Result<Matrix<T>, MatrixError> {
        let dtype = T::as_dtype();
        let row_len = vec[0].len();
        let num_rows = vec.len();
        let data = vec.concat();
        for row in vec {
            if row.len() != row_len {
                Err(MatrixError::InhomogenousShape())?;
            }
        }
        Ok( Matrix { shape:vec![row_len, num_rows], array:data, dtype } )
    }

    /// creates a 2-dimensional matrix from an array of arrays
    pub fn from_2darray<const M:usize, const N:usize>(arr:[[T;M];N]) -> Matrix<T> {
        let dtype = T::as_dtype();
        Matrix {shape:vec![M, N], array:arr.concat(), dtype}
    }

    /// creates a 3-dimensional matrix from an array of arrays of arrays
    pub fn from_3darray<const M:usize, const N:usize, const O:usize>(arr:[[[T;M];N];O]) -> Matrix<T> {
        let dtype = T::as_dtype();
        Matrix {shape:vec![M, N, O], array:arr.concat().concat(), dtype}
    }

    /// returns an identity matrix of order N
    pub fn identity(order:usize) -> Matrix<T> {
        let arr = vec![T::zero(); order*order];
        let mut identity_mat = Matrix {shape:vec![order, order], array:arr, dtype:T::as_dtype()};
        for i in 0..order {
            identity_mat[[i, i]] = T::one();
        }
        identity_mat
    }

    /// returns a null matrix of shape [usize; k]
    pub fn null<const K:usize>(shape:[usize; K]) -> Matrix<T> {
        let arr = vec![T::zero(); shape.iter().product()];
        Matrix { shape: shape.to_vec(), array: arr, dtype: T::as_dtype() }
    }
    /// returns a null matrix of shape given by by Vec<usize>
    pub fn null_from_vec(shape:Vec<usize>) -> Matrix<T> {
        let arr = vec![T::zero(); shape.iter().product()];
        Matrix { shape: shape.to_vec(), array: arr, dtype: T::as_dtype() }
    }
}