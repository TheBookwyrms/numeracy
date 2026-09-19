use std::{assert_eq, fmt::Debug};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S1<const M:usize>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S2<const M:usize, const N:usize>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S3<const M:usize, const N:usize, const O:usize>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S4<const M:usize, const N:usize, const O:usize, const P:usize>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S5<const M:usize, const N:usize, const O:usize, const P:usize, const Q:usize>;


pub trait ShapeTrait<const NDIMS:usize, const PRODUCT:usize>:PartialEq+Debug+Copy+Clone {
    fn get_self() -> Self;
    fn product(&self) -> usize;
    //fn ndims(&self) -> usize;
    fn get_index(&self, idx:usize) -> Option<usize>;
    fn as_array(&self) -> [usize;NDIMS];
    //fn from_array<const ARR:[usize;NDIMS]>() -> Self;
    //fn swap_axes<U:ShapeTrait<NDIMS>, const AX1:usize, const AX2:usize>(&self) -> U;
    fn assert_array_matches(arr:[usize; NDIMS]);
}


impl<const M:usize> ShapeTrait<1, M> for S1<M> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 1]) { assert_eq!(Self::get_self().as_array(), arr); }
    fn product(&self) -> usize { M }
    fn get_index(&self, idx:usize) -> Option<usize> {
        match idx {
            0 => Some(M),
            _ => None,
        }
    }
    fn as_array(&self) -> [usize;1] { [ M ] }
    //fn from_array<const ARR:[usize;1]>() -> Self {
    //    S1::<{ARR[0]}>
    //}
}
impl<const M:usize, const N:usize> ShapeTrait<2, {M*N}> for S2<M, N> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 2]) { assert_eq!(Self::get_self().as_array(), arr); }
    fn product(&self) -> usize { M*N }
    fn get_index(&self, idx:usize) -> Option<usize> {
        match idx {
            0 => Some(M),
            1 => Some(N),
            _ => None,
        }
    }
    fn as_array(&self) -> [usize;2] { [ M, N ] }
}
impl<const M:usize, const N:usize, const O:usize> ShapeTrait<3, {M*N*O}> for S3<M, N, O> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 3]) { assert_eq!(Self::get_self().as_array(), arr); }
    fn product(&self) -> usize { M*N*O }
    fn get_index(&self, idx:usize) -> Option<usize> {
        match idx {
            0 => Some(M),
            1 => Some(N),
            2 => Some(O),
            _ => None,
        }
    }
    fn as_array(&self) -> [usize;3] { [ M, N, O ] }
}
impl<const M:usize, const N:usize, const O:usize, const P:usize> ShapeTrait<4, {M*N*O*P}> for S4<M, N, O, P> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 4]) { assert_eq!(Self::get_self().as_array(), arr); }
    fn product(&self) -> usize { M*N*O*P }
    fn get_index(&self, idx:usize) -> Option<usize> {
        match idx {
            0 => Some(M),
            1 => Some(N),
            2 => Some(O),
            3 => Some(P),
            _ => None,
        }
    }
    fn as_array(&self) -> [usize;4] { [ M, N, O, P ] }
}
impl<const M:usize, const N:usize, const O:usize, const P:usize, const Q:usize> ShapeTrait<5, {M*N*O*P*Q}> for S5<M, N, O, P, Q> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 5]) { assert_eq!(Self::get_self().as_array(), arr); }
    fn product(&self) -> usize { M*N*O*P*Q }
    fn get_index(&self, idx:usize) -> Option<usize> {
        match idx {
            0 => Some(M),
            1 => Some(N),
            2 => Some(O),
            3 => Some(P),
            4 => Some(Q),
            _ => None,
        }
    }
    fn as_array(&self) -> [usize;5] { [ M, N, O, P, Q ] }
}
