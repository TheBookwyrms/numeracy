use crate::vectors::vector::Vector;
use crate::traits::IntoDataType;

use std::ops::{Index, IndexMut};
use std::fmt::{Debug, Display};

impl<T:Clone> Index<usize> for Vector<T> {
    type Output = T;
    /// indexes a vector by its indices
    fn index(&self, idx:usize) -> &Self::Output {
        &self.array[idx]
    }
}

impl<T:Clone> IndexMut<usize> for Vector<T> {
    /// mutably indexes a vector by indices
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T:Display + Debug + PartialEq + IntoDataType + Clone> Display for Vector<T> {
    /// format implementation for vector
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.array.as_slice())
    }
}


impl<T> Vector<T> {
    pub fn num_items(&self) -> usize {
        self.array.len()
    }

    pub fn as_ptr(&self) -> *const T {
        self.array.as_ptr()
    }

    /// get the size in memory of one item of the matrix's type T
    pub fn dtype_memsize(&self) -> usize {
        let type_size = std::mem::size_of::<T>();
        type_size
    }

    /// get the amount of memory used by the matrix as a whole
    pub fn memory_size(&self) -> usize {
        let type_size = std::mem::size_of::<T>();
        let num_items = self.array.len();
        num_items*type_size
    }
}