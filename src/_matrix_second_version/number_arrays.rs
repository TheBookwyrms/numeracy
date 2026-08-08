use crate::_vectors_first_version::Vector;
use crate::{_matrix_second_version::matrix::Matrix};
use crate::traits::Numerical;
use crate::_matrix_second_version::enums::MatrixError;
use std::ops::{Add, Mul, Neg, Sub, AddAssign};


impl<T:Numerical, const N:usize> Add<Matrix<T, N>> for Matrix<T, N> {
    type Output = Result<Matrix<T, N>, MatrixError<N>>;

    /// add two matrices together element-wise
    fn add(self, other: Self) -> Result<Matrix<T, N>, MatrixError<N>> {
        if self.shape != other.shape {
            Err(MatrixError::InvalidShapes([self.shape, other.shape]))
        } else {

            let mut v = self.array;
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other.array[idx]);
            
            Ok(Matrix {shape:self.shape, array:v })
        }
    }
}
impl<T:Numerical, const N:usize> AddAssign<Matrix<T, N>> for Matrix<T, N> {
    /// add two matrices together element-wise
    fn add_assign(&mut self, other: Self) {
        if self.shape != other.shape {
            Err(MatrixError::InvalidShapes([self.shape, other.shape])).unwrap()
        } else {
            self.array.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other.array[idx]);
        }
    }
}

impl<T:Numerical, const N:usize> Add<T> for Matrix<T, N> {
    type Output = Matrix<T, N>;
    /// adds an element to all items of a matrix
    fn add(self, other: T) -> Matrix<T, N> {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = *val+other);
        
        Matrix {shape:self.shape, array:v }
    }
}

impl<T:Numerical, const N:usize> Sub<Matrix<T, N>> for Matrix<T, N> {
    type Output = Result<Self, MatrixError<N>>;

    /// subtracts two matrices element-wise
    fn sub(self, other: Self) -> Result<Self, MatrixError<N>> {
        if self.ndims() != other.ndims() {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if self.shape != other.shape {
            Err(MatrixError::InvalidShapes([self.shape, other.shape]))
        } else {

            let mut v = self.array;
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-other.array[idx]);

            Ok(Matrix {shape:self.shape, array:v })
        }
    }
}
impl<T:Numerical, const N:usize> Sub<T> for Matrix<T, N> {
    type Output = Matrix<T, N>;
    /// subtracts an element to all items of a matrix
    fn sub(self, other: T) -> Matrix<T, N> {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = *val-other);
        
        Matrix {shape:self.shape, array:v }
    }
}

impl<T:Numerical + Neg<Output = T>, const N:usize> Neg for Matrix<T, N> {
    type Output = Matrix<T, N>;
    /// returns the matrix where every element is its negative self
    fn neg(self) -> Matrix<T, N> {

        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = -T::one() *  *val);
        
        Matrix {shape:self.shape, array:v }
    }
}

impl <T:Numerical, const N:usize> Mul<T> for Matrix<T, N> {
    type Output = Matrix<T, N>;
    /// returns the matrix where every element is multiplied by the other
    fn mul(self, other: T) -> Self::Output {
        let mut v = self.array;
        v.iter_mut().for_each(|val| *val = other * *val);
        
        Matrix {shape:self.shape, array:v }
    }
}




impl<T:Numerical + Neg<Output = T>, const N:usize> Matrix<T, N> {
    
    /// performs the dot product of two vectors (1D matrices) 
    pub fn dot(&self, other:&Self) -> Result<T, MatrixError<N>> {
        let v1 = Vector::from_slice(&self.array);
        let v2 = Vector::from_slice(&other.array);

        let dot = v1.dot(&v2)?;
        Ok(dot)
    }

    /// multiplies every element of an n-dimensional matrix by a scalar value
    pub fn multiply_by_constant(self, scalar:T) -> Matrix<T, N> {
        let mut narr = self.array;
        (0..narr.len()).for_each(|i| narr[i] *= scalar);
        Matrix {shape:self.shape, array:narr }
    }

}

impl<T:Numerical + Neg<Output = T>> Matrix<T, 2> {
    /// performs the matrix multiplication of 2 2D matrices
    pub fn matmul(&self, other:&Self) -> Result<Matrix<T, 2>, MatrixError<2>> {
        if !(self.shape[0]==other.shape[1]) {
            Err(MatrixError::InvalidShapes([self.shape, other.shape]))
        } else {
            let mut rows = vec![];
            for r in 0..self.shape[1] {
                let mut this_row = vec![];
                let row = self.get_row(r)?;

                for c in 0..other.shape[0] {
                    let col = &other.get_col(c)?;
                    this_row.push(row.dot(&col)?);
                }
                rows.extend(this_row);
            }

            Ok(Matrix {shape:[other.shape[0], self.shape[1]], array:rows })
        }
    }
    
    /// gets the minor of a matrix for row i and column j
    pub fn minor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError<2>> {
        if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape))
        } else {
            let minor = self.without_rc(row_i, col_j)?.laplace_expansion();
            minor
        }
    }

    /// gets the cofactor of a matrix for row i and column j
    pub fn cofactor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError<2>> {
        if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape))
        } else {
            let minor = self.without_rc(row_i, col_j)?.laplace_expansion()?;

            let r = row_i;
            let c = col_j;

            let cofactor_multiplier = if (r+1)+(c+1) %2 == 0 { T::one() } else { -T::one() };

            let cofactor = cofactor_multiplier * minor;
            
            // // FIX
            // let cofactor = (-T::one()).pow((r+1)+(c+1)) * minor;
            Ok(cofactor)
        }
    }

    /// get the determinant of a matrix via laplace expansion
    pub fn laplace_expansion(&self) -> Result<T, MatrixError<2>> {
        if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape))
        } else if self.shape[0] == 2 {
            let a = self[[0, 0]];
            let b = self[[0, 1]];
            let c = self[[1, 0]];
            let d = self[[1, 1]];
            Ok(a*d - b*c)
        } else {
            let row_i = 0;
            let mut determinant_sum = T::zero();

            for (col_j, col_val) in self.array[0..self.shape[0]].iter().enumerate() {
                let cofactor = self.cofactor(row_i, col_j)?;
                determinant_sum += *col_val * cofactor;
            }
            Ok(determinant_sum)
        }
    }

    /// get the matrix of cofactors of the original matrix
    pub fn cofactor_matrix(&self) -> Result<Matrix<T, 2>, MatrixError<2>> {
        if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape))
        } else {
            let num_cofactors = self.num_items();
            let mut cofactors = Vec::with_capacity(num_cofactors);
            for i in 0..num_cofactors {
                let indices = self.indices_of(i);
                cofactors.push(self.cofactor(indices[0], indices[1])?);
                //cofactors[i] = self.cofactor(indices[0], indices[1])?;
            }

            // transposed because of swapped linear algebra indexing conventions
            Ok(Matrix {shape:self.shape, array:cofactors }.transpose())
        }
    }

    /// determines if the column j of a matrix is null (zero)
    pub fn col_is_null(&self, col_j:usize) -> Result<bool, MatrixError<2>> {
        let column = self.get_col(col_j)?;
        let zeroes = (0..column.array.len()).map(|i| column.array[i]==T::zero()).all(|b| b==true);
        Ok(zeroes)
    }
}

impl<T:Numerical + Neg<Output = T>> Matrix<T, 1> {
    pub fn cross_product(&self, other:&Matrix<T, 1>) -> Result<Matrix<T, 1>, MatrixError<1>> {
        let v1 = Vector::from_slice(&self.array);
        let v2 = Vector::from_slice(&other.array);

        let mat = Matrix::from_vector(v1.cross_product(&v2)?);
        Ok(mat)
    }
}