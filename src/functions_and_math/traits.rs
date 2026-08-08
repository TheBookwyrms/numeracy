use std::fmt::Debug;

use crate::{functions_and_math::{enums::{MathItem, MathItemStruct, ValueNature, VariableNature}, function::MathTree}, matrices::Matrix, matrices2, vectors::Vector};


pub trait TraitValueNature:Clone+Debug {
    fn value_nature() -> Self;
    fn value_nature_enum<U:TraitValueNature>(nature:ValueNature) -> U;
    //fn as_trait_type<U:TraitValueNature>(&self) -> U;
}
impl TraitValueNature for ScalarValued {
    fn value_nature() -> Self { ScalarValued {} }
    fn value_nature_enum<U:TraitValueNature>(nature:ValueNature) -> U {
        match nature {
            ValueNature::ScalarValued => U::value_nature_enum(nature),
            ValueNature::VectorValued => U::value_nature_enum(nature),
            ValueNature::MatrixValued => U::value_nature_enum(nature),
        }
    }
}
impl TraitValueNature for VectorValued {
    fn value_nature() -> Self { VectorValued {} }
    fn value_nature_enum<U:TraitValueNature>(nature:ValueNature) -> U {
        match nature {
            ValueNature::ScalarValued => U::value_nature_enum(nature),
            ValueNature::VectorValued => U::value_nature_enum(nature),
            ValueNature::MatrixValued => U::value_nature_enum(nature),
        }
    }
}
impl TraitValueNature for MatrixValued {
    fn value_nature() -> Self { MatrixValued {} }
    fn value_nature_enum<U:TraitValueNature>(nature:ValueNature) -> U {
        match nature {
            ValueNature::ScalarValued => U::value_nature_enum(nature),
            ValueNature::VectorValued => U::value_nature_enum(nature),
            ValueNature::MatrixValued => U::value_nature_enum(nature),
        }
    }
}



#[derive(Debug, Clone)]
pub struct ScalarValued {}
#[derive(Debug, Clone)]
pub struct VectorValued {}
#[derive(Debug, Clone)]
pub struct MatrixValued {}





pub trait MathValue : Debug+Sized+Clone {
    type U:MathValue;
    type V:TraitValueNature;
    fn as_math_item_struct(self) -> MathItemStruct<Self::U>;
    fn as_math_tree(self) -> MathTree<Self::U, Self::V>;
}


impl<T:MathValue, R:TraitValueNature+Debug+Clone> MathValue for MathItemStruct<T> {
    type U = T;
    type V = R;
    fn as_math_item_struct(self) -> MathItemStruct<Self::U> {
        self
    }
    fn as_math_tree(self) -> MathTree<Self::U, Self::V> {
        MathTree::new(self.get_item(), self.get_thing(), Self::V::value_nature(), vec![])
    }
}

impl<T:MathValue, R:TraitValueNature+Clone+Debug> MathValue for MathTree<T, R> {
    type U = T;
    type V = R;
    fn as_math_tree(self) -> Self { self }
}

impl<T:MathValue+Clone> MathValue for Matrix<T> {
    type U = T;
    type V = MatrixValued;
    fn as_math_tree(self) -> MathTree<T, Self::V> {
        MathTree::new(MathItem::Matrix(self) , vec![])
    }
}

//impl<T:MathValue+Clone+PartialEq, const N:usize> MathValue for matrices2::Matrix<T, N> {
//    type U = T;
//    type V = MatrixValued;
//    fn as_math_item(self) -> MathTree<T, Self::V> {
//        MathTree::new(MathItem::Matrix2(self, N) , vec![])
//    }
//}

impl<T:MathValue+Clone+PartialEq> MathValue for Vector<T> {
    type U = T;
    type V = VectorValued;
    fn as_math_tree(self) -> MathTree<T, Self::V> {
        MathTree::new(MathItem::Vector(self) , vec![])
    }
}


pub trait Variable<T:MathValue, U:TraitValueNature> {
    fn as_variable(self) -> MathTree<T, U>;
}

impl<T:MathValue, U:TraitValueNature> Variable<T, U> for char {
    fn as_variable(self) -> MathTree<T, U> {
        MathTree::new(MathItem::Variable(self, U::value_nature()), vec![])
    }
}

impl<T:MathValue, U:TraitValueNature> Variable<T, U> for &str {
    fn as_variable(self) -> MathTree<T, U> {
        let mut chars = self.chars();
        let first_char = chars.nth(0).unwrap();
        let remaining_chars = chars;
        assert!(remaining_chars.count()==0);
        MathTree::new(MathItem::Variable(first_char, U::value_nature()) , vec![])
    }
}

impl<T:MathValue, U:TraitValueNature> Variable<T, U> for String {
    fn as_variable(self) -> MathTree<T, U> {
        let mut chars = self.chars();
        let first_char = chars.nth(0).unwrap();
        let remaining_chars = chars;
        assert!(remaining_chars.count()==0);
        MathTree::new(MathItem::Variable(first_char, U::value_nature()) , vec![])
    }
}

//impl<T:MathValue> MathValue for &str {
//    type U = T;
//    fn as_math_item(self) -> MathTree<T> {
//        let mut chars = self.chars();
//        let first_char = chars.nth(0).unwrap();
//        let remaining_chars = chars;
//        assert!(remaining_chars.count()==0);
//        MathTree::new(MathItem::Variable(first_char) , vec![])
//    }
//}
//
//impl MathValue for String {
//    type U = Self;
//    fn as_math_item(self) -> MathTree<Self> {
//        let mut chars = self.chars();
//        let first_char = chars.nth(0).unwrap();
//        let remaining_chars = chars;
//        assert!(remaining_chars.count()==0);
//        MathTree::new(MathItem::Variable(first_char) , vec![])
//    }
//}


macro_rules! impl_MathValue_for_number {
    ($value_type:ty) => {
        //impl MathValue for $value_type {
        impl MathValue for $value_type {
            type U = $value_type;
            type V = ScalarValued;
            fn as_math_item(self) -> MathTree<$value_type, Self::V> {
                MathTree::new(MathItem::Number(self), vec![])
            }
        }
    };
}


impl_MathValue_for_number!(f32);
impl_MathValue_for_number!(f64);
impl_MathValue_for_number!(i8);
impl_MathValue_for_number!(i16);
impl_MathValue_for_number!(i32);
impl_MathValue_for_number!(i64);
impl_MathValue_for_number!(i128);
impl_MathValue_for_number!(isize);
impl_MathValue_for_number!(u8);
impl_MathValue_for_number!(u16);
impl_MathValue_for_number!(u32);
impl_MathValue_for_number!(u64);
impl_MathValue_for_number!(u128);
impl_MathValue_for_number!(usize);