use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Vector {
    pub(crate) tuple: Tuple,
}

impl Vector {
    const VECTOR_W: f64 = 0.0;
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            tuple: Tuple::new(x, y, z, Vector::VECTOR_W),
        }
    }

    pub fn x(&self) -> f64 {
        self.tuple.x
    }

    pub fn y(&self) -> f64 {
        self.tuple.y
    }

    pub fn z(&self) -> f64 {
        self.tuple.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector::new(
            self.x() / magnitude,
            self.y() / magnitude,
            self.z() / magnitude,
        )
    }

    #[allow(dead_code)]
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

use std::cmp::PartialEq;
impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.tuple == other.tuple
    }
}

use std::ops::Add;
impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        let result = &self.tuple + &other.tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

use std::ops::Sub;
impl<'a, 'b> Sub<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &'b Vector) -> Vector {
        let result = &self.tuple - &other.tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

use std::ops::Neg;
impl<'a> Neg for &'a Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        let result = -&self.tuple;
        Vector::new(result.x, result.y, result.z)
    }
}

use std::ops::Mul;
impl<'a> Mul<f64> for &'a Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        let result = &self.tuple * scalar;
        Vector::new(result.x, result.y, result.z)
    }
}

use std::ops::Div;
impl<'a> Div<f64> for &'a Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Vector {
        let result = &self.tuple / scalar;
        Vector::new(result.x, result.y, result.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn creating_a_vector_with_w_0() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.tuple.w, Vector::VECTOR_W);
    }

    #[test]
    fn comparing_two_points() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn comparing_two_different_points() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_ne!(v1, v2);
    }

    #[test]
    fn comparing_two_points_with_a_small_difference() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(1.0, 2.0, 3.000001);
        assert_eq!(v1, v2);
    }

    #[test]
    fn adding_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(&v1 + &v2, Vector::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(&v1 - &v2, Vector::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(&zero - &v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-&v, Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(&v * 3.5, Vector::new(3.5, 7.0, 10.5));
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(&v * 0.5, Vector::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(&v / 2.0, Vector::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_1_0() {
        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_0_1() {
        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn computing_the_magnitude_of_vector_neg_1_neg_2_neg_3() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Vector::new(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vector::new(1.0, -2.0, 1.0));
    }
}
