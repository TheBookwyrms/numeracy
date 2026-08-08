use crate::functions_and_math::traits::{MathValue, MatrixValued, ScalarValued, TraitValueNature, VectorValued};
use crate::matrices::Matrix;
use crate::matrices2;
use crate::vectors::Vector;

//#[derive(Clone, Copy, Debug)]
//pub enum IndeterminateForm {
//    ZeroMultipliedByInfinity,
//    ZeroDividedByZero
//}

#[derive(Debug, Clone)]
pub struct MathItemStruct<T:MathValue+Clone> {
    item:MathItem,
    thing:T,
}
impl<T:MathValue> MathItemStruct<T> {
    pub fn new(item:MathItem, thing:T) -> Self {
        Self { item, thing }
    }
    pub fn get_item(&self) -> MathItem {
        self.item
    }
    pub fn get_thing(self) -> T {
        self.thing
    }
}


#[derive(Clone, Debug, Copy)]
pub enum MathItem {
    Number,
    Variable,
    Operation,
    //Indeterminate,
    Matrix,
    Matrix2,
    Vector,
}
//impl<T:MathValue> MathItem<T> {
//    pub fn get_value_nature<U:TraitValueNature>(&self) -> U {
//        match self {
//            Self::Number(_num) => U::value_nature(),
//            Self::Variable(_var) => U::value_nature(),
//            Self::Operation(_op) => U::value_nature(),
//            Self::Matrix(_mat) => U::value_nature(),
//            Self::Vector(_vec) => U::value_nature(),
//        }
//    }
//}

#[derive(Debug, Clone, Copy)]
pub enum VariableNature {
    Constant,
    SingleVariable,
    MultiVariable,
}

#[derive(Debug, Clone, Copy)]
pub enum ValueNature {
    ScalarValued,
    VectorValued,
    MatrixValued,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,

    Cosine,
    Sine,
    Tangent,
    Secant,
    Cosecant,
    Cotangent,

    ArcCosine,
    ArcSine,
    ArcTangent,
    ArcSecant,
    ArcCosecant,
    ArcCotangent,

    Log10,
    Ln,

    //DotProduct,
    //CrossProduct,
    //MatrixMultiplication,

    AbsoluteValue,
    Floor,
    Ceiling,
    FloorDivision,
    RemainderDivision,
}


impl Operation {
    pub fn is_differentiable(&self) -> bool {
        match self {
            Operation::Addition => true,
            Operation::Subtraction => true,
            Operation::Multiplication => true,
            Operation::Division => true,
            Operation::Exponentiation => true,
            
            Operation::Cosine => true,
            Operation::Sine => true,
            Operation::Tangent => true,
            Operation::Secant => true,
            Operation::Cosecant => true,
            Operation::Cotangent => true,

            Operation::ArcCosine => true,
            Operation::ArcSine => true,
            Operation::ArcTangent => true,
            Operation::ArcSecant => true,
            Operation::ArcCosecant => true,
            Operation::ArcCotangent => true,

            Operation::Log10 => true,
            Operation::Ln => true,

            Operation::AbsoluteValue => false,
            Operation::Floor => false,
            Operation::Ceiling => false,
            Operation::FloorDivision => false,
            Operation::RemainderDivision => false,
        }
    }
    pub fn symbol(&self) -> &str {
        match self {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Multiplication => "*",
            Operation::Division => "/",
            Operation::Exponentiation => "^",
            
            Operation::Cosine => "cos",
            Operation::Sine => "sin",
            Operation::Tangent => "tan",
            Operation::Secant => "sec",
            Operation::Cosecant => "csc",
            Operation::Cotangent => "cot",

            Operation::ArcCosine => "arccos",
            Operation::ArcSine => "arcsin",
            Operation::ArcTangent => "arctan",
            Operation::ArcSecant => "arcsec",
            Operation::ArcCosecant => "arccsc",
            Operation::ArcCotangent => "arccot",

            Operation::Log10 => "log10",
            Operation::Ln => "ln",

            Operation::AbsoluteValue => "|",
            Operation::Floor => "floor",
            Operation::Ceiling => "ceil",
            Operation::FloorDivision => "floordiv",
            Operation::RemainderDivision => "remainder",
        }
    }
}


#[derive(Debug)]
pub enum FunctionError {
    NoOperationChildren,
    IncorrectOperationChildrenQuantity,
}