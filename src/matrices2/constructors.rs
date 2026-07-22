use crate::matrices2::matrix::Matrix;
use crate::traits::Numerical;
use crate::enums::MatrixError;
use crate::vectors::Vector;

impl<T> Matrix<T, 1> {
    /// creates a 1-dimensional matrix from a Vector
    pub fn from_vector(vector:Vector<T>) -> Matrix<T, 1> {
        Matrix {shape:[vector.num_items()], array: vector.array}
        //Matrix { shape: vec![vector.num_items()], array: vector.array }
    }

    /// creates a 1-dimensional matrix from a vec
    pub fn from_vec(vec:Vec<T>) -> Matrix<T, 1> {
        Matrix {shape:[vec.len()], array: vec}
    }
}

impl<T:Clone> Matrix<T, 1> {
    /// creates a 1-dimensional matrix from a slice
    pub fn from_slice(slice:&[T]) -> Matrix<T, 1> {
        Matrix {shape:[slice.len()], array: slice.to_vec()}
    }

    /// creates a 1-dimensional matrix from an array
    pub fn from_1darray<const M:usize>(arr:[T;M]) -> Matrix<T, 1> {
        Matrix {shape:[arr.len()], array:arr.to_vec()}
    }
}

impl<T:Clone> Matrix<T, 2> {
    /// creates a 2-dimensional matrix from a vec of vecs
    pub fn from_vec_of_vec(vec:Vec<Vec<T>>) -> Result<Matrix<T, 2>, MatrixError> {
        let row_len = vec[0].len();
        let num_rows = vec.len();
        let data = vec.concat();
        for row in vec {
            if row.len() != row_len {
                Err(MatrixError::InhomogenousShape())?;
            }
        }
        Ok( Matrix { shape:[row_len, num_rows], array:data } )
    }

    /// creates a 2-dimensional matrix from an array of arrays
    pub fn from_2darray<const M:usize, const N:usize>(arr:[[T;M];N]) -> Matrix<T, 2> {
        Matrix {shape:[M, N], array:arr.concat()}
    }
}

impl<T:Clone> Matrix<T, 3> {
    /// creates a 3-dimensional matrix from an array of arrays of arrays
    pub fn from_3darray<const M:usize, const N:usize, const O:usize>(arr:[[[T;M];N];O]) -> Matrix<T, 3> {
        Matrix {shape:[M, N, O], array:arr.concat().concat() }
    }
}





impl<T:Numerical> Matrix<T, 1> {
    /// creates matrix from a scalar value
    pub fn from_scalar(f:T) -> Matrix<T, 1> {
        Matrix {shape:[1], array:vec![f]}
    }
}

impl<T:Numerical> Matrix<T, 2> {
    /// returns an identity matrix of order N
    pub fn identity(order:usize) -> Matrix<T, 2> {
        let arr = vec![T::zero(); order*order];
        let mut identity_mat = Matrix {shape:[order, order], array:arr };
        for i in 0..order {
            identity_mat[[i, i]] = T::one();
        }
        identity_mat
    }
}

impl<T:Numerical, const N:usize> Matrix<T, N> {
    /// returns a null matrix of shape [usize; k]
    pub fn null(shape:[usize; N]) -> Matrix<T, N> {
        let arr = vec![T::zero(); shape.iter().product()];
        Matrix { shape: shape, array: arr }
    }
}