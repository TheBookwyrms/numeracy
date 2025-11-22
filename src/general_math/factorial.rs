use std::ops::Range;

use crate::traits::Uint;

pub fn factorial<T:Uint>(n:T) -> T  where  Range<T>: Iterator<Item = T> {
    let mut p = T::one();
    for i in T::range(T::one(), n+T::one()) {
        p *= i;
    }
    p
}