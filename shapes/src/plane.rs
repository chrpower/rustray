use crate::{Intersection, Material, Shape, SHAPE_ID};
use core::{Point, Vector};
use math::{Matrix4, Ray};
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct Plane {
    id: usize,
    #[allow(dead_code)]
    transform: Matrix4,
    inverse_transform: Matrix4,
    material: Material,
}

impl Plane {
    pub fn new(transform: Matrix4, material: Material) -> Self {
        let id = SHAPE_ID.fetch_add(1, Ordering::SeqCst);
        let inverse_transform = transform.inverse();
        Self {
            id,
            transform,
            inverse_transform,
            material,
        }
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = ray.transform(&self.inverse_transform);

        if transformed_ray.direction.y().abs() < 0.0001 {
            return vec![];
        }

        let t = -transformed_ray.origin.y() / transformed_ray.direction.y();
        vec![Intersection::new(t, self)]
    }

    fn normal_at(&self, _world_point: &Point) -> Vector {
        let local_normal = Vector::new(0.0, 1.0, 0.0);
        let world_normal = &self.inverse_transform.transpose() * &local_normal;
        world_normal.normalize()
    }
}

impl Shape for Plane {
    fn id(&self) -> &usize {
        &self.id
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        self.intersect(ray)
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        self.normal_at(world_point)
    }

    fn get_inverse_transform(&self) -> &Matrix4 {
        &self.inverse_transform
    }

    fn colour_at(&self, world_point: &Point) -> core::Colour {
        self.material.pattern.colour_at_object(self, world_point)
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new(Matrix4::identity(), Material::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::Plane;
    use core::{Point, Vector};
    use math::Ray;

    mod intersect {
        use super::*;

        #[test]
        fn ray_parallel_to_plane() {
            let p = Plane::default();
            let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 0);
        }

        #[test]
        fn ray_coplanar_to_plane() {
            let p = Plane::default();
            let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 0);
        }

        #[test]
        fn ray_from_above() {
            let p = Plane::default();
            let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 1);
            assert_eq!(xs[0].t, 1.0);
        }

        #[test]
        fn ray_from_below() {
            let p = Plane::default();
            let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 1);
            assert_eq!(xs[0].t, 1.0);
        }
    }
}
