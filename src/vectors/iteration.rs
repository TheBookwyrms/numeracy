use crate::vectors::Vector;


pub struct VectorIterator<'a, T> {
    vectors: &'a Vector<T>,
    index: usize,
}

pub struct VectorIntoIterator<T> {
    vectors:Vector<T>,
}

impl<'a, T> Iterator for VectorIterator<'a, T> {
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

impl<T> Iterator for VectorIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vectors.array.len() == 0 {
            return None;
        }
        let result = self.vectors.array.remove(0);
        Some(result)
    }
}


impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = VectorIntoIterator<T>;
    
    fn into_iter(self) -> VectorIntoIterator<T> {
        VectorIntoIterator { vectors: self }
    }
}


impl<T> Vector<T> {
    pub fn iter(&'_ self) -> VectorIterator<'_, T> {
        VectorIterator { vectors: self, index: 0 }
    }
}
