use crate::matrices::matrix::Matrix;
use crate::enums::MatrixDataTypes;



impl PartialEq for Matrix<u32> {
    fn eq(&self, other: &Self) -> bool {
        self.shape==other.shape && self.array==other.array && self.dtype==other.dtype
    }
}

impl From<Matrix<u32>> for Matrix<u64>{
    fn from(mat:Matrix<u32>) -> Matrix<u64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as u64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::U64 }
    }
}

impl From<Matrix<u32>> for Matrix<u128>{
    fn from(mat:Matrix<u32>) -> Matrix<u128> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as u128).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::U128 }
    }
}


impl From<Matrix<u32>> for Matrix<i64>{
    fn from(mat:Matrix<u32>) -> Matrix<i64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::I64 }
    }
}

impl From<Matrix<u32>> for Matrix<i128>{
    fn from(mat:Matrix<u32>) -> Matrix<i128> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as i128).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::I128 }
    }
}

impl From<Matrix<u32>> for Matrix<usize>{
    fn from(mat:Matrix<u32>) -> Matrix<usize> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as usize).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::USIZE }
    }
}

impl From<Matrix<u32>> for Matrix<f32>{
    fn from(mat:Matrix<u32>) -> Matrix<f32> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f32).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::F32 }
    }
}

impl From<Matrix<u32>> for Matrix<f64>{
    fn from(mat:Matrix<u32>) -> Matrix<f64> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i] as f64).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:MatrixDataTypes::F64 }
    }
}