use crate::functions_and_math::enums::{Operation, MathItem};
use crate::functions_and_math::function::{MathTree};

pub fn add<T>(left:MathTree<T>, right:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Addition), children: Some(vec![left, right]) }
}

pub fn subtract<T>(left:MathTree<T>, right:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Subtraction), children: Some(vec![left, right]) }
}

pub fn multiply<T>(left:MathTree<T>, right:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Multiplication), children: Some(vec![left, right]) }
}

pub fn divide<T>(left:MathTree<T>, right:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Division), children: Some(vec![left, right]) }
}

pub fn pow<T>(base:MathTree<T>, exponent:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Exponentiation), children: Some(vec![base, exponent]) }
}


pub fn cos<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Cosine), children: Some(vec![theta]) }
}
pub fn sin<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Sine), children: Some(vec![theta]) }
}
pub fn tan<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Tangent), children: Some(vec![theta]) }
}
pub fn sec<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Secant), children: Some(vec![theta]) }
}
pub fn csc<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Cosecant), children: Some(vec![theta]) }
}
pub fn cot<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::Cotangent), children: Some(vec![theta]) }
}


pub fn acos<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcCosine), children: Some(vec![theta]) }
}
pub fn asin<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcSine), children: Some(vec![theta]) }
}
pub fn atan<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcTangent), children: Some(vec![theta]) }
}
pub fn asec<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcSecant), children: Some(vec![theta]) }
}
pub fn acsc<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcCosecant), children: Some(vec![theta]) }
}
pub fn acot<T>(theta:MathTree<T>) -> MathTree<T> {
    MathTree { item:MathItem::Operation(Operation::ArcCotangent), children: Some(vec![theta]) }
}