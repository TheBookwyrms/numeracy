use crate::functions_and_math::enums::{Operation, MathItem};
use crate::functions_and_math::function::{MathTree};
use crate::functions_and_math::traits::{MathValue, ScalarValued, TraitValueNature};


pub fn number<T:MathValue>(num:T) -> MathTree<T, ScalarValued> {
    MathTree::new(MathItem::Number(num), vec![])
}

pub fn variable<T:MathValue, U:TraitValueNature>(var:char) -> MathTree<T, U> {
    MathTree::new(MathItem::Variable(var, U::value_nature()), vec![])
}


pub fn add<TL:MathValue<U=F, V=V>, TR:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(left:TL, right:TR) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Addition, V::value_nature()), vec![left.as_math_tree(), right.as_math_tree()])
}

pub fn subtract<TL:MathValue<U=F, V=V>, TR:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(left:TL, right:TR) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Subtraction, V::value_nature()), vec![left.as_math_tree(), right.as_math_tree()])
}

pub fn multiply<TL:MathValue<U=F, V=V>, TR:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(left:TL, right:TR) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Multiplication, V::value_nature()), vec![left.as_math_tree(), right.as_math_tree()])
}

pub fn divide<TL:MathValue<U=F, V=V>, TR:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(numerator:TL, denominator:TR) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Division, V::value_nature()), vec![numerator.as_math_tree(), denominator.as_math_tree()])
}

pub fn pow<TL:MathValue<U=F, V=V>, TR:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(base:TL, exponent:TR) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Exponentiation, V::value_nature()), vec![base.as_math_tree(), exponent.as_math_tree()])
}


pub fn cos<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Cosine, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn sin<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Sine, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn tan<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Tangent, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn sec<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Secant, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn csc<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Cosecant, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn cot<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Cotangent, V::value_nature()), vec![theta.as_math_tree()])
}


pub fn acos<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::ArcCosine, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn asin<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::ArcSine, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn atan<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::ArcTangent, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn asec<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::ArcSecant, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn acsc<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::ArcCosecant, V::value_nature()), vec![theta.as_math_tree()])
}
pub fn acot<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(theta:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::ArcCotangent, V::value_nature()), vec![theta.as_math_tree()])
}


pub fn log10<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(val:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Log10, V::value_nature()), vec![val.as_math_tree()])
}
pub fn ln<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(val:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Ln, V::value_nature()), vec![val.as_math_tree()])
}


pub fn abs<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(val:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::AbsoluteValue, V::value_nature()), vec![val.as_math_tree()])
}
pub fn floor<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(val:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Floor, V::value_nature()), vec![val.as_math_tree()])
}
pub fn ceil<T:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(val:T) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::Ceiling, V::value_nature()), vec![val.as_math_tree()])
}
pub fn floor_divide<TL:MathValue<U=F, V=V>, TR:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(numerator:TL, denominator:TR) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::FloorDivision, V::value_nature()), vec![numerator.as_math_tree(), denominator.as_math_tree()])
}
pub fn remainder_divide<TL:MathValue<U=F, V=V>, TR:MathValue<U=F, V=V>, F:MathValue, V:TraitValueNature>(numerator:TL, denominator:TR) -> MathTree<F, V> {
    MathTree::new(MathItem::Operation(Operation::RemainderDivision, V::value_nature()), vec![numerator.as_math_tree(), denominator.as_math_tree()])
}