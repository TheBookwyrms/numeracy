use std::{fmt::Display, ops::{Add, Sub, Mul, Div, Neg}};

use crate::functions_and_math::{enums::{FunctionError, MathItem, Operation, ValueNature, VariableNature}, traits::Variable};
use crate::functions_and_math::traits::MathValue;
use crate::traits::Float;

pub struct Function<T:MathValue> {
    pub input_type: VariableNature,
    pub output_type: ValueNature,
    pub output: Vec<MathTree<T>>,
}




#[derive(Debug, Clone)]
pub struct MathTree<T:MathValue> {
    item:MathItem<T>,
    children:Option<Vec<MathTree<T>>>,
}

impl<T:MathValue> MathTree<T> {
    pub fn new(item:MathItem<T>, children:Option<Vec<MathTree<T>>>) -> MathTree<T> {
        MathTree { item, children }
    }
    pub fn get_variables(&self) -> Vec<&char> {
        let mut variables = vec![];
        match &self.item {
            MathItem::Variable(var) => {
                variables.push(var);
            },
            MathItem::Operation(op) => {
                if let Some(children) = &self.children {
                    for child in children {
                        variables.extend(child.get_variables());
                    }
                } else { /* do nothing */ }
            },
            MathItem::Matrix(matrix) => {
                panic!();
            }
            MathItem::Vector(vector) => {
                panic!();
            }
            MathItem::Indeterminate(_) => {}
            MathItem::Number(_) => {},
        }
        variables.sort_unstable();
        variables.dedup();
        variables
    }
    pub fn get_variable_nature(&self) -> VariableNature {
        let variables = self.get_variables();
        match variables.len() {
            0 => VariableNature::Constant,
            1 => VariableNature::SingleVariable,
            _ => VariableNature::MultiVariable,
        }
    }
}

impl<T:MathValue> Add for MathTree<T> {
    type Output = MathTree<T>;
    fn add(self, rhs: Self) -> Self::Output {
        MathTree { item:MathItem::Operation(Operation::Addition), children: Some(vec![self.as_math_item(), rhs.as_math_item()]) }
    }
}

impl<T:MathValue> Sub for MathTree<T> {
    type Output = MathTree<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        MathTree { item:MathItem::Operation(Operation::Subtraction), children: Some(vec![self.as_math_item(), rhs.as_math_item()]) }
    }
}

impl<T:MathValue> Mul for MathTree<T> {
    type Output = MathTree<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        MathTree { item:MathItem::Operation(Operation::Multiplication), children: Some(vec![self.as_math_item(), rhs.as_math_item()]) }
    }
}

impl<T:MathValue> Div for MathTree<T> {
    type Output = MathTree<T>;
    fn div(self, rhs: Self) -> Self::Output {
        MathTree { item:MathItem::Operation(Operation::Division), children: Some(vec![self.as_math_item(), rhs.as_math_item()]) }
    }
}

impl<T:MathValue<U=T>+Float> Neg for MathTree<T> {
    type Output = MathTree<T>;
    fn neg(self) -> Self::Output {
        MathTree { item:MathItem::Operation(Operation::Multiplication), children: Some(vec![(-T::one()).as_math_item(), self.as_math_item()]) }
    }
}





impl<T:MathValue> Display for MathTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.item {
            MathItem::Number(num) => {
                write!(f, "{:?}", num)?;
            },
            MathItem::Variable(var) => {
                write!(f, "{}", var)?;
            },
            MathItem::Operation(op) => {
                if let Some(children) = &self.children {
                    let num_children = children.len();
                    match op {
                        Operation::Addition       | Operation::Multiplication => {
                            let vec_of_string = children.into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
                            let string = vec_of_string.join(op.symbol());
                            write!(f, "({})", string)?;
                        },
                        Operation::Subtraction    | Operation::Division      |
                        Operation::Exponentiation | Operation::FloorDivision |
                        Operation::RemainderDivision => {
                            if num_children == 2 {
                                let vec_of_string = children.into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
                                let string = vec_of_string.join(op.symbol());
                                write!(f, "({})", string)?;
                            } else {
                                write!(f, "{:?}", FunctionError::IncorrectOperationChildrenQuantity)?;
                            }
                        },
                        Operation::Cosine        | Operation::Sine         |
                        Operation::Tangent       | Operation::Secant       |
                        Operation::Cosecant      | Operation::Cotangent    |
                        Operation::ArcCosine     | Operation::ArcSine      |
                        Operation::ArcTangent    | Operation::ArcSecant    |
                        Operation::ArcCosecant   | Operation::ArcCotangent |
                        Operation::Log10         | Operation::Ln           |
                        Operation::AbsoluteValue | Operation::Floor        |
                        Operation::Ceiling => {
                            if num_children == 1 {
                                let vec_of_string = children.into_iter().map(|c| c.to_string()).collect::<Vec<String>>();
                                
                                let childs_children_option = &children[0].children;
                                let use_brackets = if let Some(childs_children) = childs_children_option {
                                    if childs_children.len() == 1 {
                                        // if the child has no children, it won't be enclosed
                                        // as such, this operation must enclose it
                                        true
                                    } else {
                                        // if the child has multiple children, they will enclose
                                        // as such, this operation doesn't need to enclose them
                                        false
                                    }
                                } else {
                                    // if the child has no children, it is singular
                                    // as such, this operation must enclose it
                                    true
                                };

                                let string = if use_brackets {
                                    [op.symbol(), "(", &vec_of_string[0], ")"].concat()
                                } else {
                                    [op.symbol(), &vec_of_string[0]].concat()
                                };
                                
                                // or just do the simple way, and let double brackets happen
                                // let string = [op.symbol(), "(", &vec_of_string[0], ")"].concat();
                                                            
                                write!(f, "{}", string)?;
                            } else {
                                write!(f, "{:?}", FunctionError::IncorrectOperationChildrenQuantity)?;
                            }
                        },
                    }
                } else {
                    write!(f, "{:?}", FunctionError::NoOperationChildren)?;
                }
            },
            MathItem::Indeterminate(form) => {
                panic!();
            }
            MathItem::Matrix(matrix) => {
                panic!();
            }
            MathItem::Vector(vector) => {
                panic!();
            }
        }
        Ok(())
    }
}
