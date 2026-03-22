use crate::functions_and_math::enums::{MathItem, ValueNature, VariableNature};


pub struct MathTree<T> {
    pub item:MathItem<T>,
    pub children:Option<Vec<MathTree<T>>>,
}

pub struct Function<T> {
    pub input_type: VariableNature,
    pub output_type: ValueNature,
    pub output: Vec<MathTree<T>>,
}