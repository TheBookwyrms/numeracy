use crate::functions_and_math::function::MathTree;
use crate::functions_and_math::common_functions::*;
use crate::functions_and_math::traits::MathValue;

pub fn functions() {
    let a = 5.as_math_item();
    let b = 6.as_math_item();
    let c = 7.as_math_item();
    println!("{}", add(a, b));
    
    let a = 5.as_math_item();
    let b = 6.as_math_item();
    let c = 7.as_math_item();
    println!("{}", add(a, add(b, c)));
    
    let a = 5.as_math_item();
    let b = 6.as_math_item();
    let c = 7.as_math_item();
    println!("{}", add(add(a, b), c));
    
    let a = 5.as_math_item();
    let b = 6.as_math_item();
    let c = 7.as_math_item();
    let mut r = add(a, b);
    r.children = Some(vec![]);
    println!("{}", add(r, c));
}