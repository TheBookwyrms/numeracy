use crate::matrix::ShapeTrait;

#[derive(Debug, Clone)]
/// struct to hold matrices of (mostly) arbitrary type T
//pub struct Matrix<const N:usize, [usize;N], T> {
//pub struct Matrix<T, U:ShapeTrait> {
pub struct Matrix<T, const NDIMS:usize, U:ShapeTrait<NDIMS>> {
    /// shape of the matrix
    /// goes from inner to outer dimensions
    /// ex: ncols before nrows for 2D matrix
    //pub shape:TUPLE,
    pub shape:U,

    /// 1D container for the n-dimensional matrix
    pub array:Vec<T>,
}