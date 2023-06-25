use crate::{Intersection, Material};
use core::{Colour, Point, Vector};
use math::{Matrix4, Ray};
use std::sync::atomic::AtomicUsize;

pub static SHAPE_ID: AtomicUsize = AtomicUsize::new(0);

pub trait Shape: std::fmt::Debug {
    fn id(&self) -> &usize;
    fn get_material(&self) -> &Material;
    fn get_inverse_transform(&self) -> &Matrix4;
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, world_point: &Point) -> Vector;
    fn colour_at(&self, world_point: &Point) -> Colour;
}
