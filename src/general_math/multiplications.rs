use std::iter::Sum;

use crate::traits::Numerical;

pub fn elementwise_multiplication<T:Numerical+Sum>(arr1:&[T], arr2:&[T]) -> Vec<T> {
    assert_eq!(arr1.len(), arr2.len());
    let mul = arr1.iter().zip(arr2).map(|(a, b)| *a * *b).collect::<Vec<T>>();
    mul
}

pub fn sum_of_multiplications<T:Numerical+Sum>(arr1:&[T], arr2:&[T]) -> T {
    elementwise_multiplication(&arr1, &arr2).into_iter().sum()
}