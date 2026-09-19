use crate::vectors::Vector;
use crate::matrices::ShapeTrait;

pub struct VectorIterator<'a, T, const LEN:usize> {
    vectors: &'a Vector<T, LEN>,
    index: usize,
}

pub struct VectorIntoIterator<T, const LEN:usize> {
    vectors:Vector<T, LEN>,
}

impl<'a, T, const LEN:usize> Iterator for VectorIterator<'a, T, LEN> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vectors.array.len() {
            let result = Some(&self.vectors.array[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl<T, const LEN:usize> Iterator for VectorIntoIterator<T, LEN> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vectors.array.len() == 0 {
            return None;
        }
        let result = self.vectors.array.remove(0);
        Some(result)
    }
}


impl<T, const LEN:usize> IntoIterator for Vector<T, LEN> {
    type Item = T;
    type IntoIter = VectorIntoIterator<T, LEN>;
    
    fn into_iter(self) -> VectorIntoIterator<T, LEN> {
        VectorIntoIterator { vectors: self }
    }
}


impl<T, const LEN:usize> Vector<T, LEN> {
    pub fn iter(&'_ self) -> VectorIterator<'_, T, LEN> {
        VectorIterator { vectors: self, index: 0 }
    }
}