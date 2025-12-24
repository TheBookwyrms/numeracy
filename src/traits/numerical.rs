use crate::traits::IntoDataType;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::iter::Sum;


/// numerical trait for numerical data types that are valid in matrix
pub trait Numerical :
        Copy + Clone
        + Display + Debug
        + IntoDataType
        + PartialEq + PartialOrd
        + Add<Output = Self> + Sub<Output = Self>
        + Mul<Output = Self> + Div<Output = Self>
        + AddAssign + SubAssign + MulAssign + DivAssign
        + Sum
    { 
        fn zero() -> Self;
        fn one() -> Self;
    }

 
impl Numerical for u8 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for u16 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for u32 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for usize {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for u64 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for u128 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}

impl Numerical for i8 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for i16 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for i32 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for isize {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for i64 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}
impl Numerical for i128 {
    fn zero() -> Self {0}
    fn one() -> Self {1}
}

impl Numerical for f32 {
    fn zero() -> Self {0.0}
    fn one() -> Self {1.0}
}
impl Numerical for f64 {
    fn zero() -> Self {0.0}
    fn one() -> Self {1.0}
}