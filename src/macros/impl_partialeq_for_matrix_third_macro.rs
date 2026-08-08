use crate::matrix::Matrix;
use crate::matrix::ShapeTrait;
use crate::general_math::comparisons::float_equality;


/// generates the implementation for PartialEq<Matrix<typename>>
/// compares two matrices elementwise
/// if the numbers are numerically exact (no small errors)
/// and if the matrices have the same shape
/// then they are equal
macro_rules! impl_matrix_partialeq_exact_values {
    ($typename:ty) => {
        impl<const NDIMS:usize, U:ShapeTrait<NDIMS>> PartialEq for Matrix<$typename, NDIMS, U> {
            fn eq(&self, other:&Matrix<$typename, NDIMS, U>) -> bool {
                self.shape == other.shape && self.array == other.array
            }
        }
    };
}

/// generates the implementation for PartialEq<Matrix<typename>>
/// compares two matrices elementwise
/// if the numbers are equal within a certain distance epsilon
/// and if the matrices have the same shape
/// then they are equal
macro_rules! impl_matrix_partialeq_float_equality {
    ($typename:ty) => {
        impl<const NDIMS:usize, U:ShapeTrait<NDIMS>> PartialEq for Matrix<$typename, NDIMS, U> {
            fn eq(&self, other:&Matrix<$typename, NDIMS, U>) -> bool {
                let shape_eq = self.shape==other.shape;
                let mut val_eq = vec![false;self.array.len()];
                val_eq.iter_mut()
                      .enumerate()
                      .for_each(|(i, b)| *b = float_equality(self.array[i], other.array[i], -5));
                shape_eq && !val_eq.contains(&false)
            }
        }
    };
}


impl_matrix_partialeq_float_equality!(f32);
impl_matrix_partialeq_float_equality!(f64);

impl_matrix_partialeq_exact_values!(bool);
impl_matrix_partialeq_exact_values!(i8);
impl_matrix_partialeq_exact_values!(i16);
impl_matrix_partialeq_exact_values!(i32);
impl_matrix_partialeq_exact_values!(i64);
impl_matrix_partialeq_exact_values!(i128);
impl_matrix_partialeq_exact_values!(isize);
impl_matrix_partialeq_exact_values!(u8);
impl_matrix_partialeq_exact_values!(u16);
impl_matrix_partialeq_exact_values!(u32);
impl_matrix_partialeq_exact_values!(u64);
impl_matrix_partialeq_exact_values!(u128);
impl_matrix_partialeq_exact_values!(usize);
impl_matrix_partialeq_exact_values!(&str);
impl_matrix_partialeq_exact_values!(String);