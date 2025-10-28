use crate::matrices::matrix::Matrix;
use crate::matrices::enums::DataTypes;


impl PartialEq for Matrix<&str> {
    fn eq(&self, other: &Self) -> bool {
        self.shape==other.shape && self.array==other.array && self.dtype==other.dtype
    }
}


impl From<Matrix<&str>> for Matrix<String> {
    fn from(mat:Matrix<&str>) -> Matrix<String> {
        let narr = (0..mat.array.len()).map(|i| mat.array[i].to_string()).collect::<Vec<_>>();
        Matrix { shape:mat.shape, array:narr, dtype:DataTypes::STRING }
    }
}