use crate::{matrices::Matrix, traits::Float};

pub struct Dataset<T:Float> {
    /// data is assumed to be in row form, ie
    /// [
    ///     [x1, x2, x3, ..., xn],
    ///     [y1, y1, y2, ..., yn],
    /// etc
    /// ]
    pub data : Matrix<T>
}

impl<T:Float> Dataset<T> {
    pub fn mean(&self) -> Matrix<T> {
        let row_len = self.data.shape[0];
        let mut new_arr = vec![];
        for row in 0..(self.data.array.len()/row_len) {
            let row_sum:T = self.data.array[row*row_len..(row+1)*row_len].to_vec().into_iter().sum();
            new_arr.push(row_sum/T::usize_to_t(row_len));
        }
        let mut new_shape = self.data.shape.clone();
        new_shape[0] = 1;
        Matrix {shape:new_shape, array:new_arr, dtype:self.data.dtype}
    }

    /*
        ADD OTHER STUFF AT SOME POINT FROM old/stats_webworks/stats.py
    */
}