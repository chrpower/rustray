use crate::{Intersection, Material};
use core::{Point, Vector};
use math::Ray;

pub trait Shape: std::fmt::Debug {
    fn equals(&self, other: &dyn Shape) -> bool;
    fn id(&self) -> &usize;
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn material(&self) -> &Material;
    fn normal_at(&self, world_point: &Point) -> Vector;
}
