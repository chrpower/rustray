use crate::Material;
use core::{Point, Vector};

pub trait Shape: std::fmt::Debug {
    fn id(&self) -> &usize;
    fn material(&self) -> &Material;
    fn normal_at(&self, world_point: &Point) -> Vector;
}
