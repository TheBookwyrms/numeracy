use crate::{functions_and_math::traits::MathValue, matrices::Matrix, traits::Numerical, vectors::Vector};

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

pub enum VariableNature {
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

    AbsoluteValue,
    Floor,
    Ceiling,
    FloorDivision,
    RemainderDivision,
}


impl Operation {
    pub fn is_differentiable(self) -> bool {
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
}


#[derive(Debug)]
pub enum FunctionError {
    NoChildrenPresent,
    NotEnoughChildrenPresent,
}