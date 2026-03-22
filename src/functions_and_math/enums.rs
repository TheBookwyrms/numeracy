pub enum MathItem<T> {
    Number(T),
    Variable(char),
    Operation(Operation),
}

pub enum VariableNature {
    SingleVariable,
    MultiVariable,
}

pub enum ValueNature {
    ScalarValued,
    VectorValued
}

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
    ModuloDivision,
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
            Operation::ModuloDivision => false,
        }
    }
}