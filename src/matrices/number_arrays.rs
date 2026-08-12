use crate::matrices::S2;
use crate::matrices::shape::{S1, ShapeTrait};
use crate::vectors::Vector;
use crate::{matrices::matrix::Matrix};
use crate::traits::Numerical;
use crate::matrices::enums::MatrixError;
use std::ops::{Add, Mul, Neg, Sub, AddAssign};


impl<T:Numerical, const NDIMS:usize, U:ShapeTrait<NDIMS>> Add<Self> for Matrix<T, NDIMS, U> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut v = self.array;
        v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other.array[idx]);
        
        Matrix {shape:self.shape, array:v }
    }
}

impl<T:Numerical, const NDIMS:usize, U:ShapeTrait<NDIMS>> AddAssign<Self> for Matrix<T, NDIMS, U> {
    /// add two matrices together element-wise
    fn add_assign(&mut self, other: Self) {
        self.array.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other.array[idx]);
    }
}

impl<T:Numerical, const NDIMS:usize, U:ShapeTrait<NDIMS>> Add<T> for Matrix<T, NDIMS, U> {
    type Output = Self;
    /// adds an element to all items of a matrix
    fn add(self, other: T) -> Self {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = *val+other);
        
        Matrix {shape:self.shape, array:v }
    }
}

impl<T:Numerical, const NDIMS:usize, U:ShapeTrait<NDIMS>> Sub<Self> for Matrix<T, NDIMS, U> {
    type Output = Self;

    /// subtracts two matrices element-wise
    fn sub(self, other: Self) -> Self {
        let mut v = self.array;
        v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-other.array[idx]);

        Matrix {shape:self.shape, array:v }
    }
}
impl<T:Numerical, const NDIMS:usize, U:ShapeTrait<NDIMS>> Sub<T> for Matrix<T, NDIMS, U> {
    type Output = Self;
    /// subtracts an element to all items of a matrix
    fn sub(self, other: T) -> Self {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = *val-other);
        
        Matrix {shape:self.shape, array:v }
    }
}

impl<T:Numerical + Neg<Output = T>, const NDIMS:usize, U:ShapeTrait<NDIMS>> Neg for Matrix<T, NDIMS, U> {
    type Output = Self;
    /// returns the matrix where every element is its negative self
    fn neg(self) -> Self {
        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = -T::one() *  *val);
        
        Matrix {shape:self.shape, array:v }
    }
}

impl <T:Numerical, const NDIMS:usize, U:ShapeTrait<NDIMS>> Mul<T> for Matrix<T, NDIMS, U> {
    type Output = Self;
    /// returns the matrix where every element is multiplied by the other
    fn mul(self, other: T) -> Self::Output {
        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = other * *val);
        
        Matrix {shape:self.shape, array:v }
    }
}


// deleted because Mul<T> works
//impl<T:Numerical + Neg<Output = T>, const NDIMS:usize, U:ShapeTrait<NDIMS>> Matrix<T, NDIMS, U> {
//    /// multiplies every element of an n-dimensional matrix by a scalar value
//    pub fn multiply_by_constant(self, scalar:T) -> Self {
//        let mut narr = self.array;
//        (0..narr.len()).for_each(|i| narr[i] *= scalar);
//        Matrix {shape:self.shape, array:narr }
//    }
//}



impl<T:Numerical + Neg<Output = T>, const M:usize> Matrix<T, 1, S1<M>> {
    
    /// performs the dot product of two vectors (1D matrices) 
    pub fn dot(&self, other:&Self) -> T {
        let v1: Vector<T, M> = Vector::from_slice(&self.get_view_of_array());
        let v2: Vector<T, M> = Vector::from_slice(&other.get_view_of_array());

        let dot = &v1.dot(&v2);
        *dot
    }
}

impl<T:Numerical + Neg<Output = T>, const M:usize, const N:usize> Matrix<T, 2, S2<M, N>> {
    /// performs the matrix multiplication of 2 2D matrices
    pub fn matmul<const U:usize>(&self, other:&Matrix<T, 2, S2<U, M>>) -> Matrix<T, 2, S2<U, N>> {
    //pub fn matmul<const U:usize>(&self, other:&Matrix<T, 2, S2<U, M>>) -> Matrix<T, 2, S2<M, M>> {
        let mut rows = vec![];
        for r in 0..N {
            let mut this_row = vec![];
            let row = self.get_row(r).unwrap();

            for c in 0..U {
                let col = other.get_col(c).unwrap();
                let row_value = row.dot(&col);
                this_row.push(row_value);
            }
            rows.extend(this_row);
        }

        Matrix {shape:S2::<U, N>, array:rows }
        //Matrix {shape:S2::<M, M>, array:rows }
    }

    /// determines if the column j of a matrix is null (zero)
    pub fn col_is_null(&self, col_j:usize) -> Result<bool, MatrixError<2>> {
        let column = self.get_col(col_j)?;
        let zeroes = (0..column.array.len()).map(|i| column.array[i]==T::zero()).all(|b| b==true);
        Ok(zeroes)
    }
}

impl<T:Numerical + Neg<Output = T>, const N:usize> Matrix<T, 2, S2<N, N>> {

    // just too hard to get working for the moment
    // because without_rc needs a type of NEW_ORDER = ORDER-1
    // and this can't be done without const generics recursively
    ///// gets the minor of a matrix for row i and column j
    //pub fn minor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError<2>> {
    //    // actually of size (M = N - 1)
    //    // reason no size parameter is given, is there is no point
    //    // the size parameter will dissapear after the laplace expansion
    //    // so just hide it with size N (even if that's a lie)
    //    let minor = self.without_rc::<S2<N, N>>(row_i, col_j)?.laplace_expansion();
    //    Ok(minor)
    //}

    // just too hard to get working for the moment
    // because without_rc needs a type of NEW_ORDER = ORDER-1
    // and this can't be done without const generics recursively
    ///// gets the cofactor of a matrix for row i and column j
    //pub fn cofactor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError<2>> {
    //    // actually of size (M = N - 1)
    //    // reason no size parameter is given, is there is no point
    //    // the size parameter will dissapear after the laplace expansion
    //    // so just hide it with size N (even if that's a lie)
    //    let minor = self.without_rc(row_i, col_j)?;
    //    let minor = minor.laplace_expansion();
    //
    //    let r = row_i;
    //    let c = col_j;
    //
    //    let cofactor_multiplier = if (r+1)+(c+1) %2 == 0 { T::one() } else { -T::one() };
    //
    //    let cofactor = cofactor_multiplier * minor;
    //    
    //    // // FIX
    //    // let cofactor = (-T::one()).pow((r+1)+(c+1)) * minor;
    //    Ok(cofactor)
    //}

    // just too hard to get working for the moment
    // because without_rc needs a type of NEW_ORDER = ORDER-1
    // and this can't be done without const generics recursively
    ///// get the determinant of a matrix via laplace expansion
    //pub fn laplace_expansion(&self) -> T {
    //    if N == 2 {
    //        let a = self[[0, 0]];
    //        let b = self[[0, 1]];
    //        let c = self[[1, 0]];
    //        let d = self[[1, 1]];
    //        a*d - b*c
    //    } else {
    //        //let row_i = 0;
    //        let mut determinant_sum = T::zero();
    //
    //        let i = self.array[0..N].iter().enumerate();
    //        for (col_j, col_val) in i {
    //            let cofactor = self.cofactor(0, col_j).unwrap();
    //            determinant_sum += *col_val * cofactor;
    //        }
    //        
    //                    panic!("HI");
    //        determinant_sum
    //    }
    //}

    // just too hard to get working for the moment
    // because without_rc needs a type of NEW_ORDER = ORDER-1
    // and this can't be done without const generics recursively
    ///// get the matrix of cofactors of the original matrix
    //pub fn cofactor_matrix(&self) -> Self {
    //    let num_cofactors = self.num_items();
    //    let mut cofactors = Vec::with_capacity(num_cofactors);
    //    for i in 0..num_cofactors {
    //        let indices = self.indices_of(i);
    //        cofactors.push(self.cofactor(indices[0], indices[1]).unwrap());
    //        //cofactors[i] = self.cofactor(indices[0], indices[1])?;
    //    }
    //
    //    // transposed because of swapped linear algebra indexing conventions
    //    Matrix {shape:self.shape, array:cofactors }.transpose()
    //}
}

impl<T:Numerical + Neg<Output = T>> Matrix<T, 1, S1<3>> {
    pub fn cross_product(&self, other:&Self) -> Self {
        let v1 = Vector::from_slice(&self.array);
        let v2 = Vector::from_slice(&other.array);

        let mat = Matrix::from_vector(v1.cross_product(&v2));
        mat
    }
}