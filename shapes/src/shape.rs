use crate::{Intersection, Material};
use core::{Point, Vector};
use math::Ray;
use std::sync::atomic::AtomicUsize;

pub static SHAPE_ID: AtomicUsize = AtomicUsize::new(0);

pub trait Shape: std::fmt::Debug {
    fn id(&self) -> &usize;
    fn get_material(&self) -> &Material;
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, world_point: &Point) -> Vector;
}
