use crate::enums::MatrixDataTypes;

/// trait representing floats
pub trait Float {
    fn usize_to_t(u:usize) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn powf(base:Self, exponent:Self) -> Self;
    fn as_dtype() -> MatrixDataTypes;
    fn epsilon(magnitude:isize) -> Self;
}
impl Float for f32 {
    fn usize_to_t(u:usize) -> Self {u as f32}
    fn zero() -> Self {0.0}
    fn one() -> Self {1.0}
    fn powf(base:f32, exponent:f32) -> Self {base.powf(exponent)}
    fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::F32}
    fn epsilon(magnitude:isize) -> Self {10.0_f32.powf(magnitude as f32)}
}
impl Float for f64 {
    fn usize_to_t(u:usize) -> Self {u as f64}
    fn zero() -> Self {0.0}
    fn one() -> Self {1.0}
    fn powf(base:f64, exponent:f64) -> Self {base.powf(exponent)}
    fn as_dtype() -> MatrixDataTypes {MatrixDataTypes::F64}
    fn epsilon(magnitude:isize) -> Self {10.0_f64.powf(magnitude as f64)}
}