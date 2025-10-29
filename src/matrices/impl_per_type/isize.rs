use crate::matrices::matrix::Matrix;
use crate::enums::MatrixDataTypes;



impl PartialEq for Matrix<isize> {
    fn eq(&self, other: &Self) -> bool {
        self.shape==other.shape && self.array==other.array && self.dtype==other.dtype
    }
}


impl From<Matrix<isize>> for Matrix<i64>{
    fn from(mat:Matrix<isize>) -> Matrix<i64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::I64 }
    }
}

impl From<Matrix<isize>> for Matrix<i128>{
    fn from(mat:Matrix<isize>) -> Matrix<i128> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i128).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::I128 }
    }
}


impl From<Matrix<isize>> for Matrix<i32>{
    fn from(mat:Matrix<isize>) -> Matrix<i32> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i32).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::I32 }
    }
}

impl From<Matrix<isize>> for Matrix<f32>{
    fn from(mat:Matrix<isize>) -> Matrix<f32> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f32).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::F32 }
    }
}

impl From<Matrix<isize>> for Matrix<f64>{
    fn from(mat:Matrix<isize>) -> Matrix<f64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::F64 }
    }
}