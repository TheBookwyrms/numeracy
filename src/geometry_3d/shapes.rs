use crate::{traits::Float, vectors::vector::Vector};

pub struct Line<T:Float> {
    pub point:Vector<T>,
    pub direction:Vector<T>
}

pub struct Plane<T:Float> {
    pub point:Vector<T>,
    pub normal:Vector<T>
}