use crate::functions_and_math::enums::{Operation, MathItem};
use crate::functions_and_math::function::{MathTree};
use crate::functions_and_math::traits::{MathValue, Variable};


pub fn number<T:MathValue>(num:T) -> MathTree<T> {
    MathTree::new(MathItem::Number(num), None)
}

pub fn variable<T:MathValue>(var:char) -> MathTree<T> {
    MathTree::new(MathItem::Variable(var), None)
}


pub fn add<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(left:TL, right:TR) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Addition), Some(vec![left.as_math_item(), right.as_math_item()]))
}

pub fn subtract<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(left:TL, right:TR) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Subtraction), Some(vec![left.as_math_item(), right.as_math_item()]))
}

pub fn multiply<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(left:TL, right:TR) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Multiplication), Some(vec![left.as_math_item(), right.as_math_item()]))
}

pub fn divide<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(numerator:TL, denominator:TR) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Division), Some(vec![numerator.as_math_item(), denominator.as_math_item()]))
}

pub fn pow<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(base:TL, exponent:TR) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Exponentiation), Some(vec![base.as_math_item(), exponent.as_math_item()]))
}


pub fn cos<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Cosine), Some(vec![theta.as_math_item()]))
}
pub fn sin<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Sine), Some(vec![theta.as_math_item()]))
}
pub fn tan<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Tangent), Some(vec![theta.as_math_item()]))
}
pub fn sec<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Secant), Some(vec![theta.as_math_item()]))
}
pub fn csc<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Cosecant), Some(vec![theta.as_math_item()]))
}
pub fn cot<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Cotangent), Some(vec![theta.as_math_item()]))
}


pub fn acos<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::ArcCosine), Some(vec![theta.as_math_item()]))
}
pub fn asin<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::ArcSine), Some(vec![theta.as_math_item()]))
}
pub fn atan<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::ArcTangent), Some(vec![theta.as_math_item()]))
}
pub fn asec<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::ArcSecant), Some(vec![theta.as_math_item()]))
}
pub fn acsc<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::ArcCosecant), Some(vec![theta.as_math_item()]))
}
pub fn acot<T:MathValue<U=F>, F:MathValue>(theta:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::ArcCotangent), Some(vec![theta.as_math_item()]))
}


pub fn log10<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Log10), Some(vec![val.as_math_item()]))
}
pub fn ln<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Ln), Some(vec![val.as_math_item()]))
}


pub fn abs<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::AbsoluteValue), Some(vec![val.as_math_item()]))
}
pub fn floor<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Floor), Some(vec![val.as_math_item()]))
}
pub fn ceil<T:MathValue<U=F>, F:MathValue>(val:T) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::Ceiling), Some(vec![val.as_math_item()]))
}
pub fn floor_divide<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(numerator:TL, denominator:TR) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::FloorDivision), Some(vec![numerator.as_math_item(), denominator.as_math_item()]))
}
pub fn remainder_divide<TL:MathValue<U=F>, TR:MathValue<U=F>, F:MathValue>(numerator:TL, denominator:TR) -> MathTree<F> {
    MathTree::new(MathItem::Operation(Operation::RemainderDivision), Some(vec![numerator.as_math_item(), denominator.as_math_item()]))
}