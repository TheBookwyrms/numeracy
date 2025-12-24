use std::ops::{AddAssign, Mul, Range};
use std::ops::{Add, MulAssign};
use std::fmt::{Debug, Display};

use crate::traits::IntoDataType;




pub trait Uint : 
        Add<Output = Self> + AddAssign
        + Mul<Output = Self> + MulAssign
        + Sized
        + PartialEq + PartialOrd
        + Copy + Clone
        + Display + Debug
        + IntoDataType
    {
    fn zero() -> Self;
    fn one() -> Self;
    fn range(left:Self, right:Self) -> Range<Self> where Self:Sized;
}

impl Uint for u8 {
    fn zero() -> Self {0}
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u16 {
    fn zero() -> Self {0}
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u32 {
    fn zero() -> Self {0}
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for usize {
    fn zero() -> Self {0}
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u64 {
    fn zero() -> Self {0}
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u128 {
    fn zero() -> Self {0}
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}