use crate::{geometry_3d::shapes::Line, traits::Float, vectors::vector::Vector};

impl<T:Float> Line<T> {
    pub fn passing_point_parallel_with(point:Vector<T>, parallel:Vector<T>) -> Line<T> {
        Line { point, direction:parallel }
    }
    pub fn passing_points(p1:Vector<T>, p2:Vector<T>) -> Line<T> {
        let direction = (p2-p1.clone()).unwrap();
        Line { point:p1, direction }
    }
}