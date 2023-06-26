use crate::{Computations, Shape};
use math::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub shape: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &'a Shape) -> Self {
        Self { t, shape }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        let Intersection { t, shape } = self;

        let point = ray.position(*t);
        let eye_v = -&ray.direction;
        let mut normal_v = shape.normal_at(&point);

        let inside = normal_v.dot(&eye_v) < 0.0;
        normal_v = if inside { -&normal_v } else { normal_v };

        Computations::new(*t, shape, point, eye_v, normal_v, inside)
    }
}

pub fn find_hit<'a>(intersections: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
    intersections
        .iter()
        .filter(|i| i.t >= 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

#[cfg(test)]
mod test {
    use crate::{Intersection, Material, Sphere};
    use core::{Point, Vector};
    use math::{Matrix4, Ray};

    mod creation {
        use super::*;

        #[test]
        fn access_t_and_shape() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(identity, material);

            let intersection = Intersection::new(3.5, &sphere);

            assert_eq!(intersection.t, 3.5);
        }
    }

    mod interset {
        use super::*;

        #[test]
        fn sets_shape() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(identity, material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(xs.len(), 2);
        }
    }

    mod hit {
        use super::*;
        use crate::find_hit;

        #[test]
        fn all_intersections_have_positive_t() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(identity, material);

            let i1 = Intersection::new(1.0, &sphere);
            let i2 = Intersection::new(2.0, &sphere);
            let intersections = vec![i1, i2];

            let hit = find_hit(&intersections);
            assert_eq!(hit.unwrap().t, i1.t);
        }

        #[test]
        fn some_intersections_have_negative_t() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(identity, material);

            let i1 = Intersection::new(-1.0, &sphere);
            let i2 = Intersection::new(1.0, &sphere);
            let intersections = vec![i1, i2];

            let hit = find_hit(&intersections);
            assert_eq!(hit.unwrap().t, i2.t);
        }

        #[test]
        fn all_intersections_have_negative_t() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(identity, material);

            let i1 = Intersection::new(-2.0, &sphere);
            let i2 = Intersection::new(-1.0, &sphere);
            let intersections = vec![i1, i2];

            let hit = find_hit(&intersections);

            assert!(hit.is_none());
        }

        #[test]
        fn always_lowest_nonnegative_intersection() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(identity, material);

            let i1 = Intersection::new(5.0, &sphere);
            let i2 = Intersection::new(7.0, &sphere);
            let i3 = Intersection::new(-3.0, &sphere);
            let i4 = Intersection::new(2.0, &sphere);
            let intersections = vec![i1, i2, i3, i4];

            let hit = find_hit(&intersections);
            assert_eq!(hit.unwrap().t, i4.t);
        }
    }

    mod prepare_computations {
        use super::*;

        #[test]
        fn prepare_computations() {
            let shape = Sphere::new(Matrix4::identity(), Material::default());
            let intersection = Intersection::new(4.0, &shape);
            let computations = intersection.prepare_computations(Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(computations.t, 4.0);
            assert_eq!(computations.point, Point::new(0.0, 0.0, -1.0));
            assert_eq!(computations.eye_v, Vector::new(0.0, 0.0, -1.0));
            assert_eq!(computations.normal_v, Vector::new(0.0, 0.0, -1.0));
        }

        #[test]
        fn prepare_computations_inside() {
            let shape = Sphere::new(Matrix4::identity(), Material::default());
            let intersection = Intersection::new(1.0, &shape);
            let computations = intersection.prepare_computations(Ray::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(computations.t, 1.0);
            assert_eq!(computations.point, Point::new(0.0, 0.0, 1.0));
            assert_eq!(computations.eye_v, Vector::new(0.0, 0.0, -1.0));
            assert_eq!(computations.normal_v, Vector::new(0.0, 0.0, -1.0));
            assert!(computations.inside);
        }
    }
}
