use crate::{Intersection, Material, Shape};
use core::{Point, Vector};
use math::{Matrix4, Ray};
use std::sync::atomic::{AtomicUsize, Ordering};

static SPHERE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Sphere<'a> {
    pub id: usize,
    pub transform: &'a Matrix4,
    inverse_transform: Matrix4,
    pub material: &'a Material,
}

impl<'a> Sphere<'a> {
    pub fn new(transform: &'a Matrix4, material: &'a Material) -> Self {
        let id = SPHERE_ID.fetch_add(1, Ordering::SeqCst);
        let inverse_transform = transform.inverse();
        Self {
            id,
            transform,
            inverse_transform,
            material,
        }
    }

    pub fn intersect(&'a self, ray: &Ray) -> [Option<Intersection<'a>>; 2] {
        let transformed_ray = ray.transform(&self.inverse_transform);
        let sphere_to_ray = transformed_ray.origin() - &Point::origin();

        let a = transformed_ray.direction().dot(transformed_ray.direction());
        let b = 2.0 * transformed_ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return [None, None];
        }

        [
            Some(Intersection::new(
                (-b - discriminant.sqrt()) / (2.0 * a),
                self,
            )),
            Some(Intersection::new(
                (-b + discriminant.sqrt()) / (2.0 * a),
                self,
            )),
        ]
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = &self.inverse_transform * world_point;
        let object_normal = &object_point - &Point::origin();
        let world_normal = &self.inverse_transform.transpose() * &object_normal;
        world_normal.normalize()
    }
}

impl<'a> Shape for Sphere<'a> {
    fn id(&self) -> &usize {
        &self.id
    }

    fn material(&self) -> &Material {
        self.material
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        self.normal_at(world_point)
    }
}

#[cfg(test)]
mod test {
    use crate::Material;
    use crate::Sphere;
    use math::Matrix4;
    use math::Transform;

    mod creation {
        use super::*;

        #[test]
        fn unique_id() {
            let identity = Matrix4::identity();
            let material = Material::default();

            assert_ne!(
                Sphere::new(&identity, &material).id,
                Sphere::new(&identity, &material).id
            );
        }

        #[test]
        fn access_transform() {
            let transform = Transform::default().translation(2.0, 3.0, 4.0).build();
            let material = Material::default();
            let sphere = Sphere::new(&transform, &material);

            assert_eq!(sphere.transform, &transform);
        }

        #[test]
        fn access_material() {
            let transform = Transform::default().translation(2.0, 3.0, 4.0).build();
            let material = Material::default();
            let sphere = Sphere::new(&transform, &material);

            assert_eq!(sphere.material, &material);
        }
    }

    mod intersect {
        use super::*;
        use core::Point;
        use core::Vector;
        use math::Ray;

        #[test]
        fn scaled() {
            let transform = Transform::default().scaling(2.0, 2.0, 2.0).build();
            let material = Material::default();
            let sphere = Sphere::new(&transform, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].as_ref().unwrap().t(), 3.0);
            assert_eq!(xs[1].as_ref().unwrap().t(), 7.0);
        }

        #[test]
        fn translated() {
            let transform = Transform::default().translation(5.0, 0.0, 0.0).build();
            let material = Material::default();
            let sphere = Sphere::new(&transform, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));
            assert!(xs[0].is_none() && xs[1].is_none());
        }

        #[test]
        fn two_points() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(xs[0].as_ref().unwrap().t(), 4.0);
            assert_eq!(xs[1].as_ref().unwrap().t(), 6.0);
        }

        #[test]
        fn tangent() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 1.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(xs[0].as_ref().unwrap().t(), 5.0);
            assert_eq!(xs[1].as_ref().unwrap().t(), 5.0);
        }

        #[test]
        fn misses() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 2.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert!(xs[0].is_none() && xs[1].is_none());
        }

        #[test]
        fn originates_inside() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].as_ref().unwrap().t(), -1.0);
            assert_eq!(xs[1].as_ref().unwrap().t(), 1.0);
        }

        #[test]
        fn originates_behind() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, 5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(xs[0].as_ref().unwrap().t(), -6.0);
            assert_eq!(xs[1].as_ref().unwrap().t(), -4.0);
        }
    }

    mod normal {
        use super::*;
        use core::Point;
        use core::Vector;

        #[test]
        fn point_on_the_x_axis() {
            let identity = Matrix4::identity();
            let material = Material::default();

            assert_eq!(
                Sphere::new(&identity, &material).normal_at(&Point::new(1.0, 0.0, 0.0)),
                Vector::new(1.0, 0.0, 0.0)
            );
        }

        #[test]
        fn point_on_the_y_axis() {
            let identity = Matrix4::identity();
            let material = Material::default();

            assert_eq!(
                Sphere::new(&identity, &material).normal_at(&Point::new(0.0, 1.0, 0.0)),
                Vector::new(0.0, 1.0, 0.0)
            );
        }

        #[test]
        fn point_on_the_z_axis() {
            let identity = Matrix4::identity();
            let material = Material::default();

            assert_eq!(
                Sphere::new(&identity, &material).normal_at(&Point::new(0.0, 0.0, 1.0)),
                Vector::new(0.0, 0.0, 1.0)
            );
        }

        #[test]
        fn nonaxial_point() {
            let identity = Matrix4::identity();
            let material = Material::default();

            assert_eq!(
                Sphere::new(&identity, &material).normal_at(&Point::new(
                    3.0_f64.sqrt() / 3.0,
                    3.0_f64.sqrt() / 3.0,
                    3.0_f64.sqrt() / 3.0
                )),
                Vector::new(
                    3.0_f64.sqrt() / 3.0,
                    3.0_f64.sqrt() / 3.0,
                    3.0_f64.sqrt() / 3.0
                )
            );
        }

        #[test]
        fn normal_is_a_normalized_vector() {
            let identity = Matrix4::identity();
            let material = Material::default();

            let n = Sphere::new(&identity, &material).normal_at(&Point::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ));

            assert_eq!(n, n.normalize());
        }

        #[test]
        fn translated() {
            let transform = Transform::default().translation(0.0, 1.0, 0.0).build();
            let material = Material::default();

            assert_eq!(
                Sphere::new(&transform, &material).normal_at(&Point::new(0.0, 1.70711, -0.70711)),
                Vector::new(0.0, 0.70711, -0.70711)
            );
        }

        #[test]
        fn rotation_z() {
            let transform = Transform::default()
                .rotation_z(std::f64::consts::PI / 5.0)
                .scaling(1.0, 0.5, 1.0)
                .build();

            let material = Material::default();

            assert_eq!(
                Sphere::new(&transform, &material).normal_at(&Point::new(
                    0.0,
                    2.0_f64.sqrt() / 2.0,
                    -2.0_f64.sqrt() / 2.0,
                )),
                Vector::new(0.0, 0.97014, -0.24254)
            );
        }
    }
}
