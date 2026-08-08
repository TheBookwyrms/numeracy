use std::{fmt::{Debug, Display}, ops::{Add, Div, Mul, Neg, Sub}};

use crate::{functions_and_math::{enums::{FunctionError, MathItem, Operation, ValueNature, VariableNature}, traits::{ScalarValued, TraitValueNature}}, traits::Numerical};
use crate::functions_and_math::traits::MathValue;
use crate::traits::Float;

//pub struct Function<T:MathValue> {
//    pub input_type: VariableNature,
//    pub output_type: ValueNature,
//    pub output: Vec<MathTree<T, U>>,
//}




#[derive(Debug, Clone)]
pub struct MathTree<T:MathValue, OUTPUT:TraitValueNature> {
    item:MathItem,
    thing:T,
    nature:OUTPUT,
    children:Vec<MathTree<T, OUTPUT>>,
}

impl<T:MathValue, U:TraitValueNature> MathTree<T, U> {
    pub fn one<V:Numerical>() -> V { V::one() }
    pub fn new(item:MathItem, thing:T, nature:U, children:Vec<MathTree<T, U>>) -> MathTree<T, U> {
        MathTree { item, thing, nature, children }
    }
    pub fn get_variables(&self) -> Vec<char> {
        let mut variables = vec![];
        match &self.item {
            MathItem::Variable(var, _nature) => {
                variables.push(*var);
            },
            MathItem::Operation(_op, _nature) => {
                for child in &self.children {
                    variables.extend(child.get_variables());
                }
            },
            MathItem::Matrix(matrix) => {
                for item in matrix.array.clone() {
                    variables.extend(item.as_math_tree().get_variables());
                }
            }
            MathItem::Vector(vector) => {
                for item in vector.array.clone() {
                    variables.extend(item.as_math_tree().get_variables());
                }
            }
            //MathItem::Indeterminate(_) => {}
            MathItem::Number(num) => {},
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
    //pub fn get_value_nature(&self) -> ValueNature {
    //    match self.item {
    //        MathItem::Number(num) => ValueNature::ScalarValued,
    //        MathItem::Variable(char) => {ValueNature::ScalarValued; panic!()},
    //        MathItem::Operation(op) => {ValueNature::ScalarValued; panic!()},
    //        MathItem::Matrix(mat) => ValueNature::ScalarValued,
    //        MathItem::Vector(vec) => ValueNature::ScalarValued,
    //    }
    //}
}

impl<T:MathValue, U:TraitValueNature> Add for MathTree<T, U> {
    type Output = MathTree<T, U>;
    fn add(self, rhs: Self) -> Self::Output {
        MathTree::new(MathItem::Operation(Operation::Addition, U::value_nature()), vec![self.as_math_tree(), rhs.as_math_tree()])
        //MathTree { item:MathItem::Operation(Operation::Addition), children: vec![self.as_math_item(), rhs.as_math_item()] }
    }
}

impl<T:MathValue, U:TraitValueNature> Sub for MathTree<T, U> {
    type Output = MathTree<T, U>;
    fn sub(self, rhs: Self) -> Self::Output {
        MathTree::new(MathItem::Operation(Operation::Subtraction, U::value_nature()), vec![self.as_math_tree(), rhs.as_math_tree()])
        //MathTree { item:MathItem::Operation(Operation::Subtraction), children: vec![self.as_math_item(), rhs.as_math_item()] }
    }
}

impl<T:MathValue, U:TraitValueNature> Mul for MathTree<T, U> {
    type Output = MathTree<T, U>;
    fn mul(self, rhs: Self) -> Self::Output {
        MathTree::new(MathItem::Operation(Operation::Multiplication, U::value_nature()), vec![self.as_math_tree(), rhs.as_math_tree()])
        //MathTree { item:MathItem::Operation(Operation::Multiplication), children: vec![self.as_math_item(), rhs.as_math_item()] }
    }
}

impl<T:MathValue, U:TraitValueNature> Div for MathTree<T, U> {
    type Output = MathTree<T, U>;
    fn div(self, rhs: Self) -> Self::Output {
        MathTree::new(MathItem::Operation(Operation::Division, U::value_nature()), vec![self.as_math_tree(), rhs.as_math_tree()])
        //MathTree { item:MathItem::Operation(Operation::Division), children: vec![self.as_math_item(), rhs.as_math_item()] }
    }
}

impl<T:MathValue<U = T, V=V>+Float, V:TraitValueNature> Neg for MathTree<T, V> {
    type Output = MathTree<T, V>;
    fn neg(self) -> Self::Output {
        MathTree::new(MathItem::Operation(Operation::Multiplication, V::value_nature()), vec![(-T::one()).as_math_tree(), self.as_math_tree()])
        //MathTree { item:MathItem::Operation(Operation::Multiplication), children: vec![(-T::one()).as_math_item(), self.as_math_item()] }
    }
}





impl<T:MathValue, U:TraitValueNature> Display for MathTree<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.item {
            MathItem::Number(num) => {
                write!(f, "{:?}", num)?;
            },
            MathItem::Variable(var, _nature) => {
                write!(f, "{}", var)?;
            },
            MathItem::Operation(op, _nature) => {
                let num_children = self.children.len();
                let children = &self.children;
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
                            
                            let childs_children_len = (children[0]).children.len();
                            let use_brackets = match childs_children_len {
                                0 => true,
                                1 => true,
                                _ => false,
                            };
                            //let use_brackets = if let Some(childs_children) = childs_children_option {
                            //    if childs_children.len() == 1 {
                            //        // if the child has no children, it won't be enclosed
                            //        // as such, this operation must enclose it
                            //        true
                            //    } else {
                            //        // if the child has multiple children, they will enclose
                            //        // as such, this operation doesn't need to enclose them
                            //        false
                            //    }
                            //} else {
                            //    // if the child has no children, it is singular
                            //    // as such, this operation must enclose it
                            //    true
                            //};

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
            },
            //MathItem::Indeterminate(form) => {
            //    panic!();
            //}
            MathItem::Matrix(matrix) => {
                write!(f, "{:?}", matrix)?;
            }
            MathItem::Vector(vector) => {
                write!(f, "{:?}", vector)?;
            }
        }
        Ok(())
    }
}
