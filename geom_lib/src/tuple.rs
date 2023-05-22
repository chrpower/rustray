#[derive(Debug)]
pub(crate) struct Tuple {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) w: f64,
}

impl Tuple {
    pub(crate) fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }
}

use std::cmp::PartialEq;
const EPSILON: f64 = 0.00001;
impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}

use std::ops::Add;
impl<'a, 'b> Add<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn add(self, other: &'b Tuple) -> Tuple {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

use std::ops::Sub;
impl<'a, 'b> Sub<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn sub(self, other: &'b Tuple) -> Tuple {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

use std::ops::Neg;
impl<'a> Neg for &'a Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

use std::ops::Mul;
impl<'a> Mul<f64> for &'a Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f64) -> Tuple {
        Tuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl<'a, 'b> Mul<&'b Tuple> for &'a Tuple {
    type Output = Tuple;

    fn mul(self, other: &'b Tuple) -> Tuple {
        Tuple::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

use std::ops::Div;
impl<'a> Div<f64> for &'a Tuple {
    type Output = Tuple;

    fn div(self, scalar: f64) -> Tuple {
        Tuple::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn creating_a_tuple() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(t1.x, 1.0);
        assert_eq!(t1.y, 2.0);
        assert_eq!(t1.z, 3.0);
        assert_eq!(t1.w, 4.0);
    }

    #[test]
    fn comparing_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn comparing_two_tuples_with_a_small_difference() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.0, 2.0, 3.0, 4.000001);
        assert_eq!(t1, t2);
    }

    #[test]
    fn comparing_two_different_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(2.0, 3.0, 4.0, 5.0);
        assert_ne!(t1, t2);
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(2.0, 3.0, 4.0, 5.0);
        assert_eq!(&t1 + &t2, Tuple::new(3.0, 5.0, 7.0, 9.0));
    }

    #[test]
    fn subtracting_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(2.0, 3.0, 4.0, 5.0);
        assert_eq!(&t1 - &t2, Tuple::new(-1.0, -1.0, -1.0, -1.0));
    }

    #[test]
    fn negating_a_tuple() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(-&t1, Tuple::new(-1.0, -2.0, -3.0, -4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(&t1 * 3.5, Tuple::new(3.5, 7.0, 10.5, 14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(&t1 * 0.5, Tuple::new(0.5, 1.0, 1.5, 2.0));
    }

    #[test]
    fn multiplying_two_tuples() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let t2 = Tuple::new(2.0, 3.0, 4.0, 5.0);
        assert_eq!(&t1 * &t2, Tuple::new(2.0, 6.0, 12.0, 20.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t1 = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(&t1 / 2.0, Tuple::new(0.5, 1.0, 1.5, 2.0));
    }
}
