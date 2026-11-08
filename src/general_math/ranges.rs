use crate::traits::Float;

/// returns evenly spaced numbers (num of them) between\
/// either [to, from] or [to, from), depending on include_end
pub fn linspace<T:Float>(from:T,to:T, num:usize, include_end:bool) -> Vec<T> {
    let dx = match include_end {
        false => (to-from)/T::usize_to_t(num),
        true => (to-from)/T::usize_to_t(num-1),
    };
    (0..num).map(|i| from + T::usize_to_t(i)*dx).collect::<Vec<T>>()
}