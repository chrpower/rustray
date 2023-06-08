use crate::Matrix4;
use core::Point;
use core::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, time: f64) -> Point {
        &self.origin + &(&self.direction * time)
    }

    pub fn transform(&self, transformation: &Matrix4) -> Self {
        Self {
            origin: transformation * &self.origin,
            direction: transformation * &self.direction,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Ray;
    use crate::Transform;
    use core::Point;
    use core::Vector;

    mod creation {
        use super::*;

        #[test]
        fn access() {
            let origin = Point::new(1.0, 2.0, 3.0);
            let direction = Vector::new(4.0, 5.0, 6.0);
            let ray = Ray::new(origin.clone(), direction.clone());

            assert_eq!(ray.origin, origin);
            assert_eq!(ray.direction, direction);
        }
    }

    mod position {
        use super::*;

        #[test]
        fn distance_at_t0() {
            assert_eq!(
                Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0)).position(0.0),
                Point::new(2.0, 3.0, 4.0)
            );
        }

        #[test]
        fn distance_at_t1() {
            assert_eq!(
                Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0)).position(1.0),
                Point::new(3.0, 3.0, 4.0)
            );
        }

        #[test]
        fn distance_at_neg_t1() {
            assert_eq!(
                Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0)).position(-1.0),
                Point::new(1.0, 3.0, 4.0)
            );
        }

        #[test]
        fn distance_at_t2point5() {
            assert_eq!(
                Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0)).position(2.5),
                Point::new(4.5, 3.0, 4.0)
            );
        }
    }

    mod transform {
        use super::*;

        #[test]
        fn translate() {
            let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0))
                .transform(&Transform::default().translation(3.0, 4.0, 5.0).build());

            assert_eq!(r.origin, Point::new(4.0, 6.0, 8.0));
            assert_eq!(r.direction, Vector::new(0.0, 1.0, 0.0));
        }

        #[test]
        fn scale() {
            let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0))
                .transform(&Transform::default().scaling(2.0, 3.0, 4.0).build());

            assert_eq!(r.origin, Point::new(2.0, 6.0, 12.0));
            assert_eq!(r.direction, Vector::new(0.0, 3.0, 0.0));
        }
    }
}
