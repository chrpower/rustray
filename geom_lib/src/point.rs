use crate::tuple::Tuple;

#[derive(Debug)]
struct Point {
    tuple: Tuple,
}

impl Point {
    const POINT_W: f64 = 1.0;
    #[allow(dead_code)]
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            tuple: Tuple::new(x, y, z, Point::POINT_W),
        }
    }

    #[allow(dead_code)]
    fn x(&self) -> f64 {
        self.tuple.x
    }

    #[allow(dead_code)]
    fn y(&self) -> f64 {
        self.tuple.y
    }

    #[allow(dead_code)]
    fn z(&self) -> f64 {
        self.tuple.z
    }
}

use std::cmp::PartialEq;
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.tuple == other.tuple
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

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
}
