use crate::Shape;

#[derive(Debug)]
pub struct Intersection<'a> {
    t: f64,
    shape: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &'a dyn Shape) -> Self {
        Self { t, shape }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn get_shape(&self) -> &dyn Shape {
        self.shape
    }
}

use std::cmp::Ordering;
pub fn find_hit<'a, I>(intersections: I) -> Option<&'a Intersection<'a>>
where
    I: IntoIterator<Item = [Option<&'a Intersection<'a>>; 2]>,
{
    intersections
        .into_iter()
        .flatten()
        .flatten()
        .filter(|intersection| intersection.t() >= 0.0)
        .min_by(|a, b| a.t().partial_cmp(&b.t()).unwrap_or(Ordering::Equal))
}

#[cfg(test)]
mod test {
    use crate::Intersection;
    use crate::Material;
    use crate::Sphere;
    use math::Matrix4;

    mod creation {
        use super::*;

        #[test]
        fn access_t_and_shape() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let intersection = Intersection::new(3.5, &sphere);

            assert_eq!(intersection.t(), 3.5);
            assert_eq!(intersection.get_shape().id(), &sphere.id);
        }
    }

    mod interset {
        use super::*;
        use core::Point;
        use core::Vector;
        use math::Ray;

        #[test]
        fn sets_shape() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let xs = sphere.intersect(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].as_ref().unwrap().get_shape().id(), &sphere.id);
            assert_eq!(xs[1].as_ref().unwrap().get_shape().id(), &sphere.id);
        }
    }

    mod hit {
        use super::*;
        use crate::find_hit;

        #[test]
        fn all_intersections_have_positive_t() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let i1 = Intersection::new(1.0, &sphere);
            let i2 = Intersection::new(2.0, &sphere);

            let xs = find_hit([[Some(&i1), Some(&i2)]]);
            assert_eq!(xs.unwrap().t(), i1.t());
        }

        #[test]
        fn some_intersections_have_negative_t() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let i1 = Intersection::new(-1.0, &sphere);
            let i2 = Intersection::new(1.0, &sphere);
            let xs = find_hit([[Some(&i1), Some(&i2)]]);

            assert_eq!(xs.unwrap().t(), i2.t());
        }

        #[test]
        fn all_intersections_have_negative_t() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let i1 = Intersection::new(-2.0, &sphere);
            let i2 = Intersection::new(-1.0, &sphere);
            let xs = find_hit([[Some(&i1), Some(&i2)]]);

            assert!(xs.is_none());
        }

        #[test]
        fn always_lowest_nonnegative_intersection() {
            let identity = Matrix4::identity();
            let material = Material::default();
            let sphere = Sphere::new(&identity, &material);

            let i1 = Intersection::new(5.0, &sphere);
            let i2 = Intersection::new(7.0, &sphere);
            let i3 = Intersection::new(-3.0, &sphere);
            let i4 = Intersection::new(2.0, &sphere);
            let xs = find_hit([[Some(&i1), Some(&i2)], [Some(&i3), Some(&i4)]]);

            assert_eq!(xs.unwrap().t(), i4.t());
        }
    }
}
