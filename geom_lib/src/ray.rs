use crate::SquareMatrix;
use core::Point;
use core::Vector;

#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }

    pub fn position(&self, time: f64) -> Point {
        &self.origin + &(&self.direction * time)
    }

    pub fn transform(&self, transformation: &SquareMatrix<4>) -> Ray {
        Self {
            origin: transformation * &self.origin,
            direction: transformation * &self.direction,
        }
    }
}
#[cfg(test)]
mod test {

    use crate::scaling;
    use crate::translation;
    use crate::Ray;
    use core::Point;
    use core::Vector;

    #[test]
    fn create_and_query_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin.clone(), direction.clone());

        assert_eq!(ray.origin(), &origin);
        assert_eq!(ray.direction(), &direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0))
            .transform(&translation(3.0, 4.0, 5.0));

        assert_eq!(*r.origin(), Point::new(4.0, 6.0, 8.0));
        assert_eq!(*r.direction(), Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0))
            .transform(&scaling(2.0, 3.0, 4.0));

        assert_eq!(*r.origin(), Point::new(2.0, 6.0, 12.0));
        assert_eq!(*r.direction(), Vector::new(0.0, 3.0, 0.0));
    }
}
