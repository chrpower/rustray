use crate::Shape;
use core::{Point, Vector};

pub struct Computations<'a> {
    pub t: f64,
    pub shape: &'a dyn Shape,
    pub point: Point,
    pub eye_v: Vector,
    pub normal_v: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl<'a> Computations<'a> {
    pub fn new(
        t: f64,
        shape: &'a dyn Shape,
        point: Point,
        eye_v: Vector,
        normal_v: Vector,
        inside: bool,
    ) -> Self {
        Self {
            t,
            shape,
            point,
            eye_v,
            normal_v,
            inside,
            over_point: &point + &(&normal_v * 0.0001),
        }
    }
}
