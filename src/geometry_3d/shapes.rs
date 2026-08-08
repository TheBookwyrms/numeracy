use crate::{traits::Float, _vectors_first_version::Vector};

pub struct Line<T:Float> {
    pub point:Vector<T>,
    pub direction:Vector<T>
}

pub struct Plane<T:Float> {
    pub point:Vector<T>,
    pub normal:Vector<T>
}