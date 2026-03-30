use std::{clone, fmt::{Debug, Display}};
use std::any::TypeId;

use crate::{functions_and_math::{enums::MathItem, function::MathTree}, matrices::Matrix, traits::Numerical, vectors::Vector};

pub trait MathValue : Display+Debug+Sized {
    type U:MathValue;
    fn as_math_item(self) -> MathTree<Self::U>;
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
        MathTree {item:MathItem::Matrix(self) , children:None}
    }
}

impl<T:MathValue+Clone+PartialEq> MathValue for Vector<T> {
    type U = T;
    fn as_math_item(self) -> MathTree<T> {
        MathTree {item:MathItem::Vector(self) , children:None}
    }
}

impl MathValue for &str {
    type U = Self;
    fn as_math_item(self) -> MathTree<Self> {
        let mut chars = self.chars();
        let first_char = chars.nth(0).unwrap();
        let remaining_chars = chars;
        assert!(remaining_chars.count()==0);
        MathTree {item:MathItem::Variable(first_char) , children:None}
    }
}

impl MathValue for String {
    type U = Self;
    fn as_math_item(self) -> MathTree<Self> {
        let mut chars = self.chars();
        let first_char = chars.nth(0).unwrap();
        let remaining_chars = chars;
        assert!(remaining_chars.count()==0);
        MathTree {item:MathItem::Variable(first_char) , children:None}
    }
}


macro_rules! impl_MathValue_for_number {
    ($value_type:ty) => {
        //impl MathValue for $value_type {
        impl MathValue for $value_type {
            type U = $value_type;
            fn as_math_item(self) -> MathTree<$value_type> {
                MathTree { item: MathItem::Number(self), children:None }
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