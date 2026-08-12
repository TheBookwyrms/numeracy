use std::fmt::Debug;

use crate::matrices::matrix::Matrix;
use crate::matrices::shape::{ShapeTrait, S1, S2, S3};
use crate::traits::Numerical;
use crate::vectors::Vector;


impl<T, const M:usize> Matrix<T, 1, S1<M>> {
    /// creates a 1-dimensional matrix from a Vector
    pub fn from_vector(vector:Vector<T, M>) -> Self {
        let a = Matrix {shape:S1::<M>, array: vector.array};
        a
    }

    /// creates a 1-dimensional matrix from a vec
    pub fn from_vec(vec:Vec<T>) -> Self {
        Matrix {shape:S1::<M>, array: vec}
    }

    /// creates a 1-dimensional matrix from an array
    pub fn from_1darray(arr:[T;M]) -> Self {
        Matrix {shape:S1::<M>, array:arr.into_iter().chain(Vec::new()).collect()}
    }
}

impl<T:Clone, const M:usize> Matrix<T, 1, S1<M>> {
    /// creates a 1-dimensional matrix from a slice
    pub fn from_slice(slice:&[T]) -> Self {
        Matrix {shape:S1::<M>, array: slice.to_vec()}
    }
}

impl<T:Clone, const M:usize, const N:usize> Matrix<T, 2, S2<M, N>> {
    /// creates a 2-dimensional matrix from an array of arrays
    pub fn from_2darray(arr:[[T;M];N]) -> Self {
        Matrix {shape:S2::<M, N>, array:arr.concat()}
    }
}

impl<T:Clone, const M:usize, const N:usize, const O:usize> Matrix<T, 3, S3<M, N, O>> {
    /// creates a 3-dimensional matrix from an array of arrays of arrays
    pub fn from_3darray(arr:[[[T;M];N];O]) -> Self {
        Matrix {shape:S3::<M, N, O>, array:arr.concat().concat() }
    }
}





impl<T:Numerical> Matrix<T, 1, S1<1>> {
    /// creates matrix from a scalar value
    pub fn from_scalar(f:T) -> Self {
        Matrix {shape:S1::<1>, array:vec![f]}
    }
}

impl<T:Numerical, const ORDER:usize> Matrix<T, 2, S2<ORDER, ORDER>> {
    /// returns an identity matrix of order N
    pub fn identity() -> Self {
        let arr = vec![T::zero(); ORDER*ORDER];
        let mut identity_mat = Matrix {shape:S2::<ORDER, ORDER>, array:arr };
        for i in 0..ORDER {
            identity_mat[[i, i]] = T::one();
        }

        identity_mat
    }
}

impl<T:Numerical, const NDIMS:usize, U:ShapeTrait<NDIMS>> Matrix<T, NDIMS, U> {
    /// returns a null matrix of shape [usize; k]
    pub fn null(shape:U) -> Self {
        let arr = vec![T::zero(); shape.product()];
        Matrix { shape: shape, array: arr }
    }
}
impl<T:Clone+Debug, const NDIMS:usize, U:ShapeTrait<NDIMS>> Matrix<T, NDIMS, U> {
    pub fn array_of<const N:usize>(mat:Self) -> [Self; N] {
        let matrices = (0..N)
            .map(|_| mat.clone())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        matrices
    }

    /// creates a 1-dimensional matrix from a vec
    pub fn from_vec_with_shape(vec:Vec<T>, shape:U) -> Self {
        Matrix {shape, array: vec}
    }
}