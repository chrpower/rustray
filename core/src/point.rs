use crate::tuple::Tuple;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub(crate) tuple: Tuple<4>,
}

impl Point {
    const POINT_W: f64 = 1.0;
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            tuple: Tuple::new([x, y, z, Point::POINT_W]),
        }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.tuple.values()[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.tuple.values()[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.tuple.values()[2]
    }

    #[inline]
    pub fn w(&self) -> f64 {
        self.tuple.values()[3]
    }

    pub fn origin() -> Point {
        Point::new(0.0, 0.0, 0.0)
    }
}

use std::ops::Add;
impl<'a, 'b> Add<&'b Vector> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Vector) -> Point {
        let result = &self.tuple + &other.tuple;
        Point::new(result.values()[0], result.values()[1], result.values()[2])
    }
}

use std::ops::Sub;
impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Vector;

    fn sub(self, other: &'b Point) -> Vector {
        let result = &self.tuple - &other.tuple;
        Vector::new(result.values()[0], result.values()[1], result.values()[2])
    }
}

impl<'a, 'b> Sub<&'b Vector> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Vector) -> Point {
        let result = &self.tuple - &other.tuple;
        Point::new(result.values()[0], result.values()[1], result.values()[2])
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
    use crate::Point;
    use crate::Vector;

    mod creation {
        use super::*;

        #[test]
        fn origin_point() {
            let p = Point::origin();
            assert_eq!(p.x(), 0.0);
            assert_eq!(p.y(), 0.0);
            assert_eq!(p.z(), 0.0);
            assert_eq!(p.w(), Point::POINT_W);
        }

        #[test]
        fn access_with_w_1() {
            let p = Point::new(1.0, 2.0, 3.0);
            assert_eq!(p.x(), 1.0);
            assert_eq!(p.y(), 2.0);
            assert_eq!(p.z(), 3.0);
            assert_eq!(p.w(), Point::POINT_W);
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn identical() {
            assert_eq!(Point::new(1.0, 2.0, 3.0), Point::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn small_difference() {
            assert_eq!(Point::new(1.0, 2.0, 3.0), Point::new(1.0, 2.0, 3.000001));
        }

        #[test]
        fn different() {
            assert_ne!(Point::new(1.0, 2.0, 3.0), Point::new(2.0, 3.0, 4.0));
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn addition_vector_and_point_gives_point() {
            assert_eq!(
                &Point::new(1.0, 2.0, 3.0) + &Vector::new(2.0, 3.0, 4.0),
                Point::new(3.0, 5.0, 7.0)
            );
        }

        #[test]
        fn subtraction_two_points_gives_vector() {
            assert_eq!(
                &Point::new(1.0, 2.0, 3.0) - &Point::new(2.0, 3.0, 4.0),
                Vector::new(-1.0, -1.0, -1.0)
            );
        }

        #[test]
        fn subtraction_vector_and_point_gives_point() {
            assert_eq!(
                &Point::new(1.0, 2.0, 3.0) - &Vector::new(2.0, 3.0, 4.0),
                Point::new(-1.0, -1.0, -1.0)
            );
        }
    }
}
