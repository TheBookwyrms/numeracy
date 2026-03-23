use crate::functions_and_math::enums::{Operation, MathItem};
use crate::functions_and_math::function::{MathTree};
use crate::traits::Numerical;

pub fn add<T:Numerical>(left:MathTree<T>, right:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Addition), children: Some(vec![left, right]) }
}

pub fn subtract<T:Numerical>(left:MathTree<T>, right:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Subtraction), children: Some(vec![left, right]) }
}

pub fn multiply<T:Numerical>(left:MathTree<T>, right:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Multiplication), children: Some(vec![left, right]) }
}

pub fn divide<T:Numerical>(numerator:MathTree<T>, denominator:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Division), children: Some(vec![numerator, denominator]) }
}

pub fn pow<T:Numerical>(base:MathTree<T>, exponent:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Exponentiation), children: Some(vec![base, exponent]) }
}


pub fn cos<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Cosine), children: Some(vec![theta]) }
}
pub fn sin<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Sine), children: Some(vec![theta]) }
}
pub fn tan<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Tangent), children: Some(vec![theta]) }
}
pub fn sec<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Secant), children: Some(vec![theta]) }
}
pub fn csc<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Cosecant), children: Some(vec![theta]) }
}
pub fn cot<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Cotangent), children: Some(vec![theta]) }
}


pub fn acos<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcCosine), children: Some(vec![theta]) }
}
pub fn asin<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcSine), children: Some(vec![theta]) }
}
pub fn atan<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcTangent), children: Some(vec![theta]) }
}
pub fn asec<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcSecant), children: Some(vec![theta]) }
}
pub fn acsc<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcCosecant), children: Some(vec![theta]) }
}
pub fn acot<T:Numerical>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcCotangent), children: Some(vec![theta]) }
}


pub fn log10<T:Numerical>(val:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Log10), children: Some(vec![val]) }
}
pub fn ln<T:Numerical>(val:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Ln), children: Some(vec![val]) }
}


pub fn abs<T:Numerical>(val:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::AbsoluteValue), children: Some(vec![val]) }
}
pub fn floor<T:Numerical>(val:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Floor), children: Some(vec![val]) }
}
pub fn ceil<T:Numerical>(val:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Ceiling), children: Some(vec![val]) }
}
pub fn floor_divide<T:Numerical>(numerator:MathTree<T>, denominator:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::FloorDivision), children: Some(vec![numerator, denominator]) }
}
pub fn remainder_divide<T:Numerical>(numerator:MathTree<T>, denominator:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::RemainderDivision), children: Some(vec![numerator, denominator]) }
}