use crate::functions_and_math::traits::MathValue;
use crate::matrices::Matrix;
use crate::vectors::Vector;

#[derive(Clone, Copy, Debug)]
pub enum IndeterminateForm {
    ZeroMultipliedByInfinity,
    ZeroDividedByZero
}

#[derive(Clone, Debug)]
pub enum MathItem<T:MathValue> {
    Number(T),
    Variable(char),
    Operation(Operation),
    Indeterminate(IndeterminateForm),
    Matrix(Matrix<T>),
    Vector(Vector<T>),
}

#[derive(Debug, Clone, Copy)]
pub enum VariableNature {
    Constant,
    SingleVariable,
    MultiVariable,
}

pub enum ValueNature {
    ScalarValued,
    VectorValued
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