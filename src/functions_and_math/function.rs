use std::fmt::Display;

use crate::{functions_and_math::enums::{FunctionError, MathItem, Operation, ValueNature, VariableNature}, traits::Numerical};

pub struct Function<T:Numerical> {
    pub input_type: VariableNature,
    pub output_type: ValueNature,
    pub output: Vec<MathTree<T>>,
}

pub struct MathTree<T:Numerical> {
    pub item:MathItem<T>,
    pub children:Option<Vec<MathTree<T>>>,
}


impl<T:Numerical> Display for MathTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.item {
            MathItem::Number(num) => {
                write!(f, "{}", num)?;
            },
            MathItem::Variable(var) => {
                write!(f, "{}", var)?;
            },
            MathItem::Operation(op) => {
                match op {
                    Operation::Addition => {
                        if let Some(children) = &self.children {
                            let num_children = children.len();
                            if num_children > 1 {
                                for (idx, child) in children.into_iter().enumerate() {
                                    write!(f, "{}", child)?;
                                    if !(idx+1 == num_children) {
                                        write!(f, "+")?;
                                    }
                                }
                            } else {
                                write!(f, "{:?}", FunctionError::NotEnoughChildrenPresent)?;
                            }
                        } else {
                            write!(f, "{:?}", FunctionError::NoChildrenPresent)?;
                        }
                    },
                    Operation::Subtraction => {

                    },
                    Operation::Multiplication => {

                    },
                    Operation::Division => {

                    },
                    Operation::Exponentiation => {

                    },
                    Operation::Cosine => {

                    },
                    Operation::Sine => {

                    },
                    Operation::Tangent => {

                    },
                    Operation::Secant => {

                    },
                    Operation::Cosecant => {

                    },
                    Operation::Cotangent => {

                    },
                    Operation::ArcCosine => {

                    },
                    Operation::ArcSine => {

                    },
                    Operation::ArcTangent => {

                    },
                    Operation::ArcSecant => {

                    },
                    Operation::ArcCosecant => {

                    },
                    Operation::ArcCotangent => {

                    },
                    Operation::Log10 => {

                    },
                    Operation::Ln => {

                    },
                    Operation::AbsoluteValue => {

                    },
                    Operation::Floor => {

                    },
                    Operation::Ceiling => {

                    },
                    Operation::FloorDivision => {

                    },
                    Operation::RemainderDivision => {

                    },
                }
            },
        }
        Ok(())
    }
}
