use crate::general_math::cartesian_product;
use crate::matrix::matrix::Matrix;
use crate::matrix::shape::{ShapeTrait, S1, S2, S3};
use crate::matrix::enums::{MatrixError};
use crate::vectors::Vector;

use std::ops::{Index, IndexMut, Range};
use std::fmt::{Debug, Display};

/// causes an error to be unwrapped
fn error(msg:String) {
    let a = true;
    let _b = match a {
        true =>Err(msg),
        false =>Ok(msg),
    }.unwrap();
}

impl<T:Display + Clone, const NDIMS:usize, U:ShapeTrait<NDIMS>> Matrix<T, NDIMS, U> {
    /// gets the longest length of any item
    /// contained by the matrix for use
    /// in determining spacing sizes during printing
    pub fn longest_item_str_len(&self) -> (usize, usize) {
        let mut length_left = 0;
        let mut length_right = 0;
        for i in &self.array {
            let l_i = i.to_string();
            let l_i : Vec<usize>= l_i.split(".").map(|u| u.chars().count()).collect();
            let (ll, lr) = match l_i.len() {
                1 => (l_i[0], 0),
                2 => (l_i[0], l_i[1]),
                _ => {error("too many decimals".to_string()); (0, 0)},
            };
            if ll > length_left {
                length_left = ll;
            }
            if lr > length_right {
                length_right = lr;
            }
        }
        (length_left, length_right)
    }
}

impl<T:Clone, const N:usize, const NDIMS:usize, U:ShapeTrait<NDIMS>> Index<[usize;N]> for Matrix<T, NDIMS, U> {
    type Output = T;

    /// indexes a matrix by its indices
    fn index(&self, idx:[usize;N]) -> &Self::Output {
        &self.array[self.linear_index_of(idx)]
    }
}

impl<T:Clone, const N:usize, const NDIMS:usize, U:ShapeTrait<NDIMS>> IndexMut<[usize;N]> for Matrix<T, NDIMS, U> {
    /// mutably indexes a matrix by indices
    fn index_mut(&mut self, index: [usize;N]) -> &mut Self::Output {
        let linear_idx = self.linear_index_of(index);
        &mut self.array[linear_idx]
    }
}

/// writes out a 2D matrix
fn write_2d_matrix<T:Display + Debug + PartialEq>(f: &mut std::fmt::Formatter<'_>,
                   x_len:usize,
                   ll_lr:(usize, usize),
                   min_idx:usize,
                   max_idx:usize,
                   arr:&Vec<T>
                ) -> std::fmt::Result {
    let (ll, lr) = ll_lr;
    for i in min_idx..max_idx {
        write!(f, "  [")?;
        for j in &arr[i*x_len..(i+1)*x_len] {
            let js = j.to_string();
            let js_vec = js.trim().split(".").collect::<Vec<_>>();

            let mut has_non_zero = false;
            if js_vec.len() == 2 {
                for i in js_vec[1].chars() {
                    if i != char::from_u32(0).unwrap() {
                        has_non_zero = true;
                    }
                }
            }

            let (nl, nr) = match js_vec.len() {
                0 => {error("invalid matrix?".to_string()); ("", "")},
                1 => {(js_vec[0], "")},
                2 => {(js_vec[0], if !has_non_zero {""} else {js_vec[1]})},
                _ => {error("invalid matrix?".to_string()); ("", "")},
            };

            write!(f, " {: >ll$}.{: <lr$}", nl, nr)?;
        }
        writeln!(f, "],")?;
    }
    write!(f, "")
}



impl<T:Display + Debug + PartialEq + Clone, const M:usize> Display for Matrix<T, 1, S1<M>> {
    /// format implementation for matrix
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.array.as_slice())
    }
}
impl<T:Display + Debug + PartialEq + Clone, const M:usize, const N:usize> Display for Matrix<T, 2, S2<M, N>> {
    /// format implementation for matrix
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        let x_len = self.shape.get_index(0).unwrap();
        let (ll, lr) = self.longest_item_str_len();
        let y_len = self.shape.get_index(1).unwrap();
        let _ = write_2d_matrix(f, x_len, (ll, lr), 0, y_len, &self.array);
        write!(f, "]")
    }
}
impl<T:Display + Debug + PartialEq + Clone, const M:usize, const N:usize, const O:usize> Display for Matrix<T, 3, S3<M, N, O>> {
    /// format implementation for matrix
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        let x_len = self.shape.get_index(0).unwrap();
        let y_len = self.shape.get_index(1).unwrap();
        let z_len = self.shape.get_index(2).unwrap();
        let (ll, lr) = self.longest_item_str_len();
        for i in 0..z_len {
            let min = i*y_len;
            let max = (i+1)*y_len;
            let _ = write_2d_matrix(f, x_len, (ll, lr), min, max, &self.array);
            if i != x_len {
                write!(f, "\n")?;
            }
        }
        write!(f, "]")
    }
}





impl<T, const NDIMS:usize, U:ShapeTrait<NDIMS>> Matrix<T, NDIMS, U> {

    /// converts linear index into corresponding matrix indices
    pub fn indices_of(&self, linear_index:usize) -> [usize;NDIMS] {

        let mut indices = self.shape.as_array();

        let mut curr_max :usize = self.shape.product();
        let mut curr_lin_idx = linear_index;

        for (i, s_size) in self.shape.as_array().iter().enumerate().rev() {
            // IMPORTANT!!!
            // The divisions here truncate the values
            // not a pure division with exact values
            // ex: 3.75 is truncated to 3
            let section_len = curr_max/s_size;
            let section = curr_lin_idx/section_len;
            curr_lin_idx -= section*section_len;
            curr_max = curr_max/s_size;
            indices[i] = section;
        }
        indices
    }

    /// turns matrix indices into corresponding linear index
    pub fn linear_index_of<const N:usize>(&self, indices:[usize;N]) -> usize {        
        let mut linear_idx = 0;
        
        for i in (0..NDIMS).into_iter().rev() {
            let mut idx_max = 1;
            for j in 0..i {
                idx_max *= self.shape.get_index(j).unwrap();
            }
            linear_idx += indices[i]*idx_max;
        }
        linear_idx
    }

    pub fn linear_index_of_slice(&self, slice_indices:&[usize]) -> usize {        
        let mut linear_idx = 0;
        
        for i in (0..NDIMS).into_iter().rev() {
            let mut idx_max = 1;
            for j in 0..i {
                idx_max *= self.shape.get_index(j).unwrap();
            }
            linear_idx += slice_indices[i]*idx_max;
        }
        linear_idx
    }

    /// replaced with shape.ndims()
    //pub fn ndims(&self) -> usize {
    //    N
    //}

    pub fn num_items(&self) -> usize {
        //self.shape.product()
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

    // deprecated because it's too difficult to make work (seemingly)
    //pub fn squeeze_axes<const NEWNDIMS:usize, V:ShapeTrait<NEWNDIMS>>(self) -> Matrix<T, NEWNDIMS, V> {
    //    let mut new_shape = [0;NEWNDIMS];
    //    self.shape.as_array().into_iter().filter(|val| val != &1).enumerate().for_each(|(idx, dim)| new_shape[idx] = dim);
    //
    //
    //
    //    //let new_shape = self.shape
    //    //                                .clone()
    //    //                                .into_iter()
    //    //                                .filter(|val| val != &1)
    //    //                                .collect::<[usize;K]>().try_into().unwrap();
    //
    //    Matrix { shape: new_shape, array: self.array }
    //}
}

impl<T, const M:usize> Matrix<T, 1, S1<M>> {
    pub fn to_vector(self) -> Vector<T, M> {
        Vector {array:self.array}
    }
}

impl<T:Clone, const NDIMS:usize, U:ShapeTrait<NDIMS>> Matrix<T, NDIMS, U> {
    /// swap two axes of an N-dimensional array
    pub fn swap_axes<V:ShapeTrait<NDIMS>>(&self, axis1:usize, axis2:usize) -> Matrix<T, NDIMS, V> {

        let swapped_arr = self.array.clone();

        //let original_shape = self.shape.as_array();
        //let mut altered_shape = self.shape.as_array();
        //altered_shape[axis1] = original_shape[axis2];
        //altered_shape[axis2] = original_shape[axis1];
        
        let mut swapped_mat = Matrix {shape:V::get_self(), array:swapped_arr };

        for index in 0..self.array.len() {

            let indices = self.indices_of(index);
            let mut swapped_indices = indices.clone();
            swapped_indices[axis1] = indices[axis2];
            swapped_indices[axis2] = indices[axis1];
            

            let new_linear_index = swapped_mat.linear_index_of(swapped_indices);

            swapped_mat.array[new_linear_index] = self.array[index].clone();
        }

        //V::assert_array_matches(altered_shape);
        
        swapped_mat
    }

    
    /// index a matrix by Ranges, returning a submatrix composed of the included bounds
    pub fn get_submatrix<V:ShapeTrait<NDIMS>>(&self, bounds:[Range<usize>;NDIMS]) -> Result<Matrix<T, NDIMS, V>, MatrixError<NDIMS>> {
        let not_bounded = bounds.iter().enumerate().map(|(i, range)| match range.clone().max() {
            Some(max) => max < self.shape.get_index(i).unwrap(),
            None => false,
        }).collect::<Vec<bool>>().contains(&false);

        if not_bounded {
            Err(MatrixError::InvalidBounds)
        } else {
            //new let iters = bounds.map(|range| range.collect::<Vec<usize>>());

            let mut new_shape = bounds.clone().map(|range| range.len());
            new_shape.reverse();

            //let new_shape = bounds.clone()
            //                                  .map(|range| range.len())
            //                                  .to_vec()
            //                                  .into_iter()
            //                                  .rev();
            //                                  .collect::<Vec<usize>>();


            let mut new_arr = vec![];
            let iters = bounds.map(|range| range.collect::<Vec<usize>>());
            let mut combinations = cartesian_product::cartesian_product(iters);
            combinations.sort();

            for indices in combinations {
                let linear_index = self.linear_index_of_slice(indices.as_slice());
                let idx_val = self.array[linear_index].clone();
                new_arr.push(idx_val);
            }

            let mat = Matrix { shape:V::get_self().reverse(), array:new_arr };
            let mat_final = mat.swap_axes::<V>(0, NDIMS-1);
            //V::assert_array_matches(mat_final.shape.as_array());
            Ok(mat_final)
        }
    }

}
impl<T:Clone, const M:usize, const N:usize> Matrix<T, 2, S2<M, N>> {  
    /// transpose a 2-dimensional matrix
    pub fn transpose(self) -> Matrix<T, 2, S2<N, M>> {
        self.swap_axes(0, 1)
        //panic!(
        //    "WAIT, this isn't the same as actual transpose with swap axes,
        //    because it just changes the reading shape,
        //    but doesn't actually perform the transposing operation"
        //);
        //Matrix { shape:S2::<N, M>, array:self.array, }
    }

    /// get row i of a matrix
    pub fn get_row(&self, idx:usize) -> Result<Matrix<T, 1,  S1<M>>, MatrixError<2>> {
        match idx<N {
            true => {
                    let row_len = M;
                    Ok(Matrix::from_slice(&self.array[(idx*row_len)..(idx+1)*row_len]))
                },
            false => Err(MatrixError::InvalidIndex(idx)),
        }
    }
    /// get column j of a matrix
    pub fn get_col(&self, idx:usize)  -> Result<Matrix<T, 1, S1<N>>, MatrixError<2>> {
        match idx<M {
            true => {
                let row_len = M;
                let nrows = N;
                let mut column = Vec::with_capacity(nrows);
                for row_i in 0..nrows {
                    column.push(self.array[row_i*row_len..(row_i+1)*row_len][idx].clone());
                }
                Ok(Matrix::from_vec(column))
            },
            false => Err(MatrixError::InvalidIndex(idx)),
        }
    }

    /// returns the matrix without the specified row and column
    pub fn without_rc<V:ShapeTrait<2>>(&self, row_i:usize, col_j:usize) -> Result<Matrix<T, 2, V>, MatrixError<2>> {
        if !(row_i<N && col_j<M) {
            Err(MatrixError::InvalidIndices([row_i, col_j]))
        } else {

            let mut new_shape = self.shape.as_array();
            //new_shape.into_iter().enumerate().map(|(idx, val)| new_shape[idx] = new_shape[idx] - 1);
            (0..2).for_each(|i| new_shape[i] = self.shape.as_array()[i]-1);
            let mut v = vec![];
            
            for row in 0..N {
                if row != row_i {
                    for (col, val) in self.get_row(row)?.array.into_iter().enumerate() {
                        if col != col_j {
                            v.push(val);
                        }
                    }
                }
            }

            //V::assert_array_matches(new_shape);
            Ok(Matrix {shape:V::get_self(), array:v })
        }
    }

    /// returns the matrix without the specified column
    pub fn without_col<const V:usize>(&self, col_j:usize) -> Result<Matrix<T, 2, S2<V, N>>, MatrixError<2>> {
        if !(col_j<M) {
            Err(MatrixError::InvalidIndex(col_j))
        } else {

            //let mut new_shape = self.shape.as_array();
            //new_shape[0] -= 1;
            let mut v = vec![];
            
            for row in 0..N {
                for (col, val) in self.get_row(row)?.array.into_iter().enumerate() {
                    if col != col_j {
                        v.push(val);
                    }
                }
            }
            //S2::<V, N>::assert_array_matches(new_shape);
            Ok(Matrix {shape:S2::<V, N>::get_self(), array:v })
        }
    }

    /// expands a matrix vertically (extending number of items per column)
    pub fn expand_vertically<const V:usize, const HEIGHT:usize>(&self, other:Matrix<T, 2, S2<M, V>>) -> Matrix<T, 2, S2<M, HEIGHT>> {


        //let (self_x_len, other_x_len) = (M, M);
        //let (self_y_len, other_y_len) = (N, V);

        //let num_terms_per_axis:usize   = self.shape.as_array()[0..1].iter().sum();
        //let num_terms_after_axis:usize = self.shape.as_array()[2..2].iter().sum();


        //let new_shape = [self_x_len, self_y_len+other_y_len];

        let mut v = vec![];
        //let mut new_shape = self.shape[0..1].to_vec();
        //new_shape.push(self_y_len+other_y_len);
        //new_shape.extend_from_slice(&self.shape[(1+1)..self.ndims()]);

        v.extend(self.array.clone());
        v.extend(other.array.clone());

        //for n in 0..(num_terms_per_axis+num_terms_after_axis) {
        //    v.extend( self.array[(n*N)..((n+1)*N)].to_vec());
        //}
        //for n in 0..(num_terms_per_axis+num_terms_after_axis) {
        //    v.extend(other.array[(n*V)..((n+1)*V)].to_vec());
        //}
        //S2::<M, HEIGHT>::assert_array_matches([M, N+V]);
        Matrix {shape:S2::<M, HEIGHT>::get_self(), array:v }
    }


    /// expands a matrix horizontally (extending number of items per row)
    pub fn expand_horizontally<const V:usize, const WIDTH:usize>(&self, other:Matrix<T, 2, S2<V, N>>) -> Matrix<T, 2, S2<WIDTH, N>> {
        //let (self_x_len, other_x_len) = (M, V);
        //let (self_y_len, other_y_len) = (N, N);

        //let _new_shape = [self_x_len+other_x_len, self_y_len];

        let mut v = vec![];

            for n in 0..N {
                v.extend( self.get_row(n).unwrap().array);
                v.extend(other.get_row(n).unwrap().array);
            }

        //S2::<WIDTH, N>::assert_array_matches([M+V, N]);
        Matrix {shape:S2::<WIDTH, N>::get_self(), array:v }
    }

    pub fn flip_vertically(self) -> Self {
        let mut narr = Vec::with_capacity(self.num_items());

        let width = M;
        let height = N;

        for i in 0..height {
            let row_slice = &self.array[(height-i-1)*width..(height-i)*width];
            narr.extend_from_slice(row_slice);
        }

        Matrix { shape: self.shape, array: narr }
    }
}
impl<T:Clone, const NDIMS:usize, U:ShapeTrait<NDIMS>> Matrix<T, NDIMS, U> {

    pub fn reshape<const NEWDIMS:usize, V:ShapeTrait<NEWDIMS>>(self, shape:V) -> Result<Matrix<T, NEWDIMS, V>, MatrixError<NEWDIMS>> {
        if self.shape.product() == shape.product() {
            Ok(Matrix {shape, array:self.array})
        } else {
            Err(MatrixError::InvalidShape(shape.as_array()))
        }
    }

    // this can also be accomplished with reshape
    //pub fn new_axis<const M:usize>(self) -> Matrix<T, M> {
    //    assert_eq!(N+1, M);
    //    let mut new_shape = [0;M];
    //    self.shape.into_iter().enumerate().for_each(|(idx, dim)| new_shape[idx] = dim);
    //    new_shape[N] = 1;
    //    //let mut new_shape = self.shape;
    //    //new_shape.push(1);
    //    Matrix {shape:new_shape, array:self.array}
    //}

    // removed because you might as well do a reshape for this purpose
    //pub fn remove_axis<const M:usize>(self, axis:usize) -> Result<Matrix<T, M>, MatrixError<M>> {
    //    assert_eq!(N-1, M);
    //
    //    let mut new_shape = [0;M];
    //    self.shape.into_iter()
    //              .enumerate()
    //              .filter(|(idx, _val)| idx != &axis)
    //              .for_each(|(_idx, val)| new_shape[_idx] = val);
    //    if self.array.len() == new_shape.iter().product() {
    //        Ok(Matrix {shape:new_shape, array:self.array})
    //    } else {
    //        Err(MatrixError::InvalidShape(new_shape))
    //    }
    //}

    pub fn get_view_of_array(&self) -> &[T] {
        &self.array
    }
}