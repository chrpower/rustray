use crate::tuple::Tuple;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Point {
    tuple: Tuple,
}

impl Point {
    const POINT_W: f64 = 1.0;
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            tuple: Tuple::new(x, y, z, Point::POINT_W),
        }
    }

    pub fn x(&self) -> f64 {
        self.tuple.x
    }

    pub fn y(&self) -> f64 {
        self.tuple.y
    }

    #[allow(dead_code)]
    pub fn z(&self) -> f64 {
        self.tuple.z
    }
}

use std::cmp::PartialEq;
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.tuple == other.tuple
    }
}

use std::ops::Add;
impl<'a, 'b> Add<&'b Vector> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Vector) -> Point {
        let result = &self.tuple + &other.tuple;
        Point::new(result.x, result.y, result.z)
    }
}

use std::ops::Sub;
impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Vector;

    fn sub(self, other: &'b Point) -> Vector {
        let result = &self.tuple - &other.tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

impl<'a, 'b> Sub<&'b Vector> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Vector) -> Point {
        let result = &self.tuple - &other.tuple;
        Point::new(result.x, result.y, result.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vector::Vector;

    #[test]
    fn creating_a_point_with_w_1() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
        assert_eq!(p.tuple.w, Point::POINT_W);
    }

    #[test]
    fn comparing_two_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p1, p2);
    }

    #[test]
    fn comparing_two_different_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(2.0, 3.0, 4.0);
        assert_ne!(p1, p2);
    }

    #[test]
    fn comparing_two_points_with_a_small_difference() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.000001);
        assert_eq!(p1, p2);
    }

    #[test]
    fn adding_a_vector_to_a_point_gives_a_point() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(&p1 + &v, Point::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn subtracting_two_points_gives_a_vector() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&p1 - &p2, Vector::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point_gives_a_point() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(&p1 - &v, Point::new(-1.0, -1.0, -1.0));
    }
}
