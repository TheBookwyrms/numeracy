use std::ops::Range;
use std::ops::{Add, MulAssign};


//  Range<T>: Iterator<Item = T>


pub trait Uint : Add<Output = Self> + MulAssign + Sized {
    fn one() -> Self;
    fn range(left:Self, right:Self) -> Range<Self> where Self:Sized;
}

impl Uint for u8 {
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u16 {
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u32 {
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for usize {
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u64 {
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}
impl Uint for u128 {
    fn one() -> Self { 1 }
    fn range(left:Self, right:Self) -> Range<Self> { left..right }
}