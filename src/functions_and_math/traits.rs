use std::fmt::Debug;

use crate::{functions_and_math::{enums::MathItem, function::MathTree}, matrices::Matrix, vectors::Vector};

pub trait MathValue : Debug+Sized {
    type U:MathValue;
    fn as_math_item(self) -> MathTree<Self::U>;
}


impl<T:MathValue> MathValue for MathItem<T> {
    type U = T;
    fn as_math_item(self) -> MathTree<Self::U> {
        MathTree::new(self, None)
    }
}

impl<T:MathValue> MathValue for MathTree<T> {
    type U = T;
    fn as_math_item(self) -> Self {
        self
    }
}
//impl<T:MathValue<MathTree<T>>> MathValue<T> for MathTree<T> {
//    fn as_math_item(self) -> Self {
//        self
//    }
//}

impl<T:MathValue+Clone+PartialEq> MathValue for Matrix<T> {
    type U = T;
    fn as_math_item(self) -> MathTree<T> {
        MathTree::new(MathItem::Matrix(self) , None)
    }
}

impl<T:MathValue+Clone+PartialEq> MathValue for Vector<T> {
    type U = T;
    fn as_math_item(self) -> MathTree<T> {
        MathTree::new(MathItem::Vector(self) , None)
    }
}


pub trait Variable<T:MathValue> {
    fn as_variable(self) -> MathTree<T>;
}

impl<T:MathValue> Variable<T> for char {
    fn as_variable(self) -> MathTree<T> {
        MathTree::new(MathItem::Variable(self), None)
    }
}

impl<T:MathValue> Variable<T> for &str {
    fn as_variable(self) -> MathTree<T> {
        let mut chars = self.chars();
        let first_char = chars.nth(0).unwrap();
        let remaining_chars = chars;
        assert!(remaining_chars.count()==0);
        MathTree::new(MathItem::Variable(first_char) , None)
    }
}

impl<T:MathValue> Variable<T> for String {
    fn as_variable(self) -> MathTree<T> {
        let mut chars = self.chars();
        let first_char = chars.nth(0).unwrap();
        let remaining_chars = chars;
        assert!(remaining_chars.count()==0);
        MathTree::new(MathItem::Variable(first_char) , None)
    }
}

//impl<T:MathValue> MathValue for &str {
//    type U = T;
//    fn as_math_item(self) -> MathTree<T> {
//        let mut chars = self.chars();
//        let first_char = chars.nth(0).unwrap();
//        let remaining_chars = chars;
//        assert!(remaining_chars.count()==0);
//        MathTree::new(MathItem::Variable(first_char) , None)
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
//        MathTree::new(MathItem::Variable(first_char) , None)
//    }
//}


macro_rules! impl_MathValue_for_number {
    ($value_type:ty) => {
        //impl MathValue for $value_type {
        impl MathValue for $value_type {
            type U = $value_type;
            fn as_math_item(self) -> MathTree<$value_type> {
                MathTree::new(MathItem::Number(self), None)
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