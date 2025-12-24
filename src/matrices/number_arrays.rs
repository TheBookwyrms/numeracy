use crate::{matrices::matrix::Matrix};
use crate::traits::Numerical;
use crate::enums::MatrixError;
use std::ops::{Add, Neg, Sub};

impl<T:Numerical> Add for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixError>;

    /// add two matrices together element-wise
    fn add(self, other: Self) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != other.ndims() {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else if self.shape != other.shape {
            Err(MatrixError::InvalidShapes([self.shape, other.shape]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val+other.array[idx]);
            
            Ok(Matrix {shape:self.shape, array:v, dtype:self.dtype})
        }
    }
}

impl<T:Numerical> Sub for Matrix<T> {
    type Output = Result<Self, MatrixError>;

    /// subtracts two matrices element-wise
    fn sub(self, other: Self) -> Result<Self, MatrixError> {
        if self.ndims() != other.ndims() {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else if self.shape != other.shape {
            Err(MatrixError::InvalidShapes([self.shape, other.shape]))
        } else {

            let mut v = self.array.clone();
            v.iter_mut().enumerate().for_each(|(idx, val)| *val = *val-other.array[idx]);

            Ok(Matrix {shape:self.shape, array:v, dtype:self.dtype})
        }
    }
}

impl<T:Numerical + Neg<Output = T>> Matrix<T> {
    
    /// performs the dot product of two vectors (1D matrices) 
    pub fn dot(&self, other:&Self) -> Result<T, MatrixError> {
        let v1 = self.as_vector()?;
        let v2 = other.as_vector()?;

        let dot = v1.dot(&v2)?;
        Ok(dot)
    }

    /// performs the matrix multiplication of 2 2D matrices
    pub fn matmul(&self, other:&Self) -> Result<Matrix<T>, MatrixError> {
        if (self.ndims() != 2) || (other.ndims() != 2) {
            Err(MatrixError::InvalidDimensions([self.ndims(), other.ndims()]))
        } else if !(self.shape[0]==other.shape[1]) {
            Err(MatrixError::InvalidShapes([self.shape.clone(), other.shape.clone()]))
        } else if self.dtype != other.dtype {
            Err(MatrixError::InvalidDataTypes([self.dtype, other.dtype]))
        } else {
            let mut rows = vec![];
            for r in 0..self.shape[1] {
                let mut this_row = vec![];
                let row = self.get_row(r)?;

                for c in 0..other.shape[1] {
                    let col = &other.clone().get_col(c)?;
                    this_row.push(row.dot(&col)?);
                }
                rows.extend(this_row);
            }

            Ok(Matrix {shape:vec![other.shape[0], self.shape[1]], array:rows, dtype:self.dtype})
        }
    }

    /// multiplies every element of an n-dimensional matrix by a scalar value
    pub fn multiply_by_constant(&self, scalar:T) -> Matrix<T> {
        let mut narr = self.array.clone();
        (0..self.array.len()).for_each(|i| narr[i] *= scalar.clone());
        Matrix {shape:self.shape.clone(), array:narr, dtype:self.dtype}
    }
    
    /// gets the minor of a matrix for row i and column j
    pub fn minor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let minor = self.without_rc(row_i, col_j)?.laplace_expansion();
            minor
        }
    }

    /// gets the cofactor of a matrix for row i and column j
    pub fn cofactor(&self, row_i:usize, col_j:usize) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
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
    pub fn laplace_expansion(&self) -> Result<T, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        } else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else if self.shape[0] == 2 {
            let a = self[[0, 0]].clone();
            let b = self[[0, 1]].clone();
            let c = self[[1, 0]].clone();
            let d = self[[1, 1]].clone();
            Ok(a*d - b*c)
        } else {
            let row_i = 0;
            let mut determinant_sum = T::zero();

            for (col_j, col_val) in self.array[0..self.shape[0]].iter().enumerate() {
                let cofactor = self.cofactor(row_i, col_j)?;
                determinant_sum += col_val.clone() * cofactor;
            }
            Ok(determinant_sum)
        }
    }

    /// get the matrix of cofactors of the original matrix
    pub fn cofactor_matrix(&self) -> Result<Matrix<T>, MatrixError> {
        if self.ndims() != 2 {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }  else if self.shape[0] != self.shape[1] {
            Err(MatrixError::InvalidShape(self.shape.clone()))
        } else {
            let mut cofactor_matrix = self.array.clone();
            for i in 0..cofactor_matrix.len() {
                let indices = self.indices_of(i);
                cofactor_matrix[i] = self.cofactor(indices[0], indices[1])?;
            }

            // transposed because of swapped linear algebra indexing conventions
            Matrix {shape:self.shape.clone(), array:cofactor_matrix, dtype:self.dtype}.transpose()
        }
    }

    /// determines if the column j of a matrix is null (zero)
    pub fn col_is_null(&self, col_j:usize) -> Result<bool, MatrixError> {
        if self.ndims() == 2 {
            let column = self.get_col(col_j)?;
            let zeroes = (0..column.array.len()).map(|i| column.array[i]==T::zero()).all(|b| b==true);
            Ok(zeroes)
        } else {
            Err(MatrixError::InvalidDimension(self.ndims()))
        }
    }

    pub fn cross_product(&self, other:&Matrix<T>) -> Result<Matrix<T>, MatrixError> {
        let v1 = self.as_vector()?;
        let v2 = other.as_vector()?;

        let mat = Matrix::from_vector(v1.cross_product(&v2)?);
        Ok(mat)
    }
}