use crate::functions_and_math::enums::{Operation, MathItem};
use crate::functions_and_math::function::{MathTree};
use crate::functions_and_math::traits::MathValue;

pub fn add<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(left:TL, right:TR) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Addition), children: Some(vec![left.as_math_item(), right.as_math_item()]) }
}

pub fn subtract<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(left:TL, right:TR) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Subtraction), children: Some(vec![left.as_math_item(), right.as_math_item()]) }
}

pub fn multiply<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(left:TL, right:TR) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Multiplication), children: Some(vec![left.as_math_item(), right.as_math_item()]) }
}

pub fn divide<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(numerator:TL, denominator:TR) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Division), children: Some(vec![numerator.as_math_item(), denominator.as_math_item()]) }
}

pub fn pow<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(base:TL, exponent:TR) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Exponentiation), children: Some(vec![base.as_math_item(), exponent.as_math_item()]) }
}


pub fn cos<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Cosine), children: Some(vec![theta.as_math_item()]) }
}
pub fn sin<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Sine), children: Some(vec![theta.as_math_item()]) }
}
pub fn tan<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Tangent), children: Some(vec![theta.as_math_item()]) }
}
pub fn sec<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Secant), children: Some(vec![theta.as_math_item()]) }
}
pub fn csc<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Cosecant), children: Some(vec![theta.as_math_item()]) }
}
pub fn cot<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Cotangent), children: Some(vec![theta.as_math_item()]) }
}


pub fn acos<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::ArcCosine), children: Some(vec![theta.as_math_item()]) }
}
pub fn asin<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::ArcSine), children: Some(vec![theta.as_math_item()]) }
}
pub fn atan<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::ArcTangent), children: Some(vec![theta.as_math_item()]) }
}
pub fn asec<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::ArcSecant), children: Some(vec![theta.as_math_item()]) }
}
pub fn acsc<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::ArcCosecant), children: Some(vec![theta.as_math_item()]) }
}
pub fn acot<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::ArcCotangent), children: Some(vec![theta.as_math_item()]) }
}


pub fn log10<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Log10), children: Some(vec![val.as_math_item()]) }
}
pub fn ln<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Ln), children: Some(vec![val.as_math_item()]) }
}


pub fn abs<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::AbsoluteValue), children: Some(vec![val.as_math_item()]) }
}
pub fn floor<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Floor), children: Some(vec![val.as_math_item()]) }
}
pub fn ceil<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::Ceiling), children: Some(vec![val.as_math_item()]) }
}
pub fn floor_divide<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(numerator:TL, denominator:TR) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::FloorDivision), children: Some(vec![numerator.as_math_item(), denominator.as_math_item()]) }
}
pub fn remainder_divide<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(numerator:TL, denominator:TR) -> MathTree<F> {
    MathTree { item:MathItem::Operation(Operation::RemainderDivision), children: Some(vec![numerator.as_math_item(), denominator.as_math_item()]) }
}