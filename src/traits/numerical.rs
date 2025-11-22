use crate::traits::IntoDataType;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::iter::Sum;


/// numerical trait for numerical data types that are valid in matrix
pub trait Numerical :
        Clone + IntoDataType + Add<Output = Self>
        + Add<Output = Self> + Sub<Output = Self>
        + Mul<Output = Self> + Div<Output = Self>
        + AddAssign + SubAssign + MulAssign + DivAssign
        + Sum
    {}
impl Numerical for u8 {}
impl Numerical for u16 {}
impl Numerical for u32 {}
impl Numerical for usize {}
impl Numerical for u64 {}
impl Numerical for u128 {}

impl Numerical for i8 {}
impl Numerical for i16 {}
impl Numerical for i32 {}
impl Numerical for isize {}
impl Numerical for i64 {}
impl Numerical for i128 {}

impl Numerical for f32 {}
impl Numerical for f64 {}