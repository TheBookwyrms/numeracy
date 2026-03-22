use crate::functions_and_math::{enums::MathItem, function::MathTree};

pub trait MathValue<T> {
    fn as_math_item(value:T) -> MathTree<T>;
}



macro_rules! impl_MathValue_for_number {
    ($value_type:ty) => {
        impl MathValue<$value_type> for $value_type {
            fn as_math_item(value:$value_type) -> MathTree<$value_type> {
                MathTree { item: MathItem::Number(value), children:None }
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