use std::ops::Neg;
use crate::traits::Numerical;


/// trait representing floats
pub trait Float : Numerical + Neg<Output = Self> {
        fn usize_to_t(u:usize) -> Self;
        fn powf(base:Self, exponent:Self) -> Self;
        fn sqrt(base:Self) -> Self;
        fn epsilon(magnitude:isize) -> Self;
}
impl Float for f32 {
    fn usize_to_t(u:usize) -> Self {u as f32}
    fn powf(base:f32, exponent:f32) -> Self {base.powf(exponent)}
    fn sqrt(base:f32) -> Self {base.powf(0.5)}
    fn epsilon(magnitude:isize) -> Self {10.0_f32.powf(magnitude as f32)}
}
impl Float for f64 {
    fn usize_to_t(u:usize) -> Self {u as f64}
    fn powf(base:f64, exponent:f64) -> Self {base.powf(exponent)}
    fn sqrt(base:f64) -> Self {base.powf(0.5)}
    fn epsilon(magnitude:isize) -> Self {10.0_f64.powf(magnitude as f64)}
}