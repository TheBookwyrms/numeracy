mod matrix;
mod shape;


mod constructors;
mod generic_arrays;

mod number_arrays;

mod float_operations;
mod opengl_arrays;

mod enums;

pub use matrix::Matrix;
pub use shape::{ShapeTrait, S1, S2, S3, S4, S5};
pub use enums::{InverseMethod, MatrixError, MatrixForm};