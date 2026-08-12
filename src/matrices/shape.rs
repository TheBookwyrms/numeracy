use std::{assert_eq, fmt::Debug};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S1<const M:usize>;
//impl<const M:usize> Reverse<1> for S1<M> {
//    type Output = S1<M>;
//    fn reverse(self) -> Self::Output { Self::Output::get_self() }
//}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S2<const M:usize, const N:usize>;
//impl<const M:usize, const N:usize> Reverse<2> for S2<M, N> {
//    type Output = S2<N, M>;
//    fn reverse(self) -> Self::Output { Self::Output::get_self() }
//}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S3<const M:usize, const N:usize, const O:usize>;
//impl<const M:usize, const N:usize, const O:usize> Reverse<3> for S3<M, N, O> {
//    type Output = S3<O, N, M>;
//    fn reverse(self) -> Self::Output { Self::Output::get_self() }
//}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S4<const M:usize, const N:usize, const O:usize, const P:usize>;
//impl<const M:usize, const N:usize, const O:usize, const P:usize> Reverse<4> for S4<M, N, O, P> {
//    type Output = S4<P, O, N, M>;
//    fn reverse(self) -> Self::Output { Self::Output::get_self() }
//}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S5<const M:usize, const N:usize, const O:usize, const P:usize, const Q:usize>;
//impl<const M:usize, const N:usize, const O:usize, const P:usize, const Q:usize> Reverse<5> for S5<M, N, O, P, Q> {
//    type Output = S5<Q, P, O, N, M>;
//    fn reverse(self) -> Self::Output { Self::Output::get_self() }
//}


//pub trait Reverse<const NDIMS:usize> {
//    type Output:Reverse<NDIMS>+ShapeTrait<NDIMS>;
//    fn reverse(self) -> Self::Output;
//}

//pub trait ShapeTrait {
pub trait ShapeTrait<const NDIMS:usize>:PartialEq+Debug+Copy+Clone {
    type ReverseOutput:ShapeTrait<NDIMS>;
    fn get_self() -> Self;
    //const PRODUCT:usize;
    fn product(&self) -> usize;
    //fn ndims(&self) -> usize;
    fn get_index(&self, idx:usize) -> Option<usize>;
    fn as_array(&self) -> [usize;NDIMS];
    //fn from_array<const ARR:[usize;NDIMS]>() -> Self;
    //fn swap_axes<U:ShapeTrait<NDIMS>>(&self, ax1:usize, ax2:usize) -> U;
    fn assert_array_matches(arr:[usize; NDIMS]);
    fn reverse(self) -> Self::ReverseOutput;
}


impl<const M:usize> ShapeTrait<1> for S1<M> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 1]) { assert_eq!(Self::get_self().as_array(), arr); }
    //const PRODUCT:usize = M;
    fn product(&self) -> usize { M }
    fn get_index(&self, idx:usize) -> Option<usize> {
        match idx {
            0 => Some(M),
            _ => None,
        }
    }
    fn as_array(&self) -> [usize;1] { [ M ] }
    type ReverseOutput = S1<M>;
    fn reverse(self) -> Self::ReverseOutput { Self::ReverseOutput::get_self() }
}
impl<const M:usize, const N:usize> ShapeTrait<2> for S2<M, N> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 2]) { assert_eq!(Self::get_self().as_array(), arr); }
    //const PRODUCT:usize = M*N;
    fn product(&self) -> usize { M*N }
    fn get_index(&self, idx:usize) -> Option<usize> {
        match idx {
            0 => Some(M),
            1 => Some(N),
            _ => None,
        }
    }
    fn as_array(&self) -> [usize;2] { [ M, N ] }
    type ReverseOutput = S2<N, M>;
    fn reverse(self) -> Self::ReverseOutput { Self::ReverseOutput::get_self() }
}
impl<const M:usize, const N:usize, const O:usize> ShapeTrait<3> for S3<M, N, O> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 3]) { assert_eq!(Self::get_self().as_array(), arr); }
    //const PRODUCT:usize = M*N*O;
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
    type ReverseOutput = S3<O, N, M>;
    fn reverse(self) -> Self::ReverseOutput { Self::ReverseOutput::get_self() }
}
impl<const M:usize, const N:usize, const O:usize, const P:usize> ShapeTrait<4> for S4<M, N, O, P> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 4]) { assert_eq!(Self::get_self().as_array(), arr); }
    //const PRODUCT:usize = M*N*O*P;
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
    type ReverseOutput = S4<P, O, N, M>;
    fn reverse(self) -> Self::ReverseOutput { Self::ReverseOutput::get_self() }
}
impl<const M:usize, const N:usize, const O:usize, const P:usize, const Q:usize> ShapeTrait<5> for S5<M, N, O, P, Q> {
    fn get_self() -> Self { Self }
    fn assert_array_matches(arr:[usize; 5]) { assert_eq!(Self::get_self().as_array(), arr); }
    //const PRODUCT:usize = M*N*O*P*Q;
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
    type ReverseOutput = S5<Q, P, O, N, M>;
    fn reverse(self) -> Self::ReverseOutput { Self::ReverseOutput::get_self() }
}
