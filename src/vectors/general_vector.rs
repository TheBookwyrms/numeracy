use crate::enums::MatrixError;
use crate::matrices::{Matrix, S1};
use crate::vectors::vector::Vector;
use crate::general_math::comparisons::max;

use std::ops::{Index, IndexMut};
use std::fmt::{Debug, Display};





impl<T:Clone, const LEN:usize> Index<usize> for Vector<T, LEN> {
    type Output = T;
    /// indexes a vector by its indices
    fn index(&self, idx:usize) -> &Self::Output {
        &self.array[idx]
    }
}

impl<T:Clone, const LEN:usize> IndexMut<usize> for Vector<T, LEN> {
    /// mutably indexes a vector by indices
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl<T:Display + Debug + PartialEq + Clone, const LEN:usize> Display for Vector<T, LEN> {
    /// format implementation for vector
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.array.as_slice())
        //write!(f, "{:?}", self.array.as_slice())
    }
}


impl<T, const LEN:usize> Vector<T, LEN> {
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

    pub fn to_matrix(self) -> Matrix<T, 1, S1<LEN>> {
        Matrix { shape: S1::<LEN>, array: self.array }
    }

    pub fn is_vec3(&self) -> bool {
        self.num_items() == 3
    }

    pub fn extend<U:IntoIterator<Item=T>>(mut self, more:U) -> Self {
        self.array.extend(more);
        self
    }
}

impl<T:Clone, const LEN:usize> Vector<T, LEN> {
    pub fn swap_items(&self, idx1:usize, idx2:usize) -> Result<Vector<T, LEN>, MatrixError> {
        if max(vec![idx1, idx2]) >= self.num_items() {
            Err(MatrixError::InvalidIndex(max(vec![idx1, idx2])))
        } else {
            let mut new_arr = self.array.clone();
            new_arr[idx1] = self.array[idx2].clone();
            new_arr[idx2] = self.array[idx1].clone();
            Ok(Vector {array:new_arr})
        }
    }
}