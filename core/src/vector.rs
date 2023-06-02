use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub(crate) tuple: Tuple<4>,
}

impl Vector {
    const VECTOR_W: f64 = 0.0;
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            tuple: Tuple::new([x, y, z, Vector::VECTOR_W]),
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

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        self - &(&(normal * 2.0) * self.dot(normal))
    }
}

use std::ops::{Add, Sub};
macro_rules! implement_vector_operations {
    ($($trait:ident, $method:ident),+) => {
        $(
            impl<'a, 'b> $trait<&'b Vector> for &'a Vector {
                type Output = Vector;

                fn $method(self, other: &'b Vector) -> Vector {
                    let result = &self.tuple.$method(&other.tuple);
                    Vector::new(result.values()[0], result.values()[1], result.values()[2])
                }
            }
        )+
    };
}

implement_vector_operations!(Add, add);
implement_vector_operations!(Sub, sub);

use std::ops::{Div, Mul};
macro_rules! implement_vector_scalar_operations {
    ($($trait:ident, $method:ident),+) => {
        $(
            impl<'a> $trait<f64> for &'a Vector {
                type Output = Vector;

                fn $method(self, scalar: f64) -> Vector {
                    let result = &self.tuple.$method(scalar);
                    Vector::new(result.values()[0], result.values()[1], result.values()[2])
                }
            }
        )+
    };
}

implement_vector_scalar_operations!(Div, div);
implement_vector_scalar_operations!(Mul, mul);

use std::ops::Neg;
impl<'a> Neg for &'a Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        let result = -&self.tuple;
        Vector::new(result.values()[0], result.values()[1], result.values()[2])
    }
}

use std::cmp::PartialEq;
impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.tuple == other.tuple
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    mod creation {
        use super::*;

        #[test]
        fn access_with_w_0() {
            let v = Vector::new(1.0, 2.0, 3.0);
            assert_eq!(v.x(), 1.0);
            assert_eq!(v.y(), 2.0);
            assert_eq!(v.z(), 3.0);
            assert_eq!(v.w(), Vector::VECTOR_W);
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn identical() {
            assert_eq!(Vector::new(1.0, 2.0, 3.0), Vector::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn small_difference() {
            assert_eq!(Vector::new(1.0, 2.0, 3.0), Vector::new(1.0, 2.0, 3.000001));
        }

        #[test]
        fn different() {
            assert_ne!(Vector::new(1.0, 2.0, 3.0), Vector::new(2.0, 3.0, 4.0));
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn addition() {
            assert_eq!(
                &Vector::new(1.0, 2.0, 3.0) + &Vector::new(2.0, 3.0, 4.0),
                Vector::new(3.0, 5.0, 7.0)
            );
        }

        #[test]
        fn subtraction() {
            assert_eq!(
                &Vector::new(1.0, 2.0, 3.0) - &Vector::new(2.0, 3.0, 4.0),
                Vector::new(-1.0, -1.0, -1.0)
            );
        }

        #[test]
        fn subtraction_vector_from_the_zero_vector() {
            let zero = Vector::new(0.0, 0.0, 0.0);
            let v = Vector::new(1.0, -2.0, 3.0);
            assert_eq!(&zero - &v, Vector::new(-1.0, 2.0, -3.0));
        }

        #[test]
        fn negation() {
            let v = Vector::new(1.0, -2.0, 3.0);
            assert_eq!(-&v, Vector::new(-1.0, 2.0, -3.0));
        }

        #[test]
        fn multiplication_by_a_scalar() {
            let v = Vector::new(1.0, 2.0, 3.0);
            assert_eq!(&v * 3.5, Vector::new(3.5, 7.0, 10.5));
        }

        #[test]
        fn multiplication_by_a_fraction() {
            let v = Vector::new(1.0, 2.0, 3.0);
            assert_eq!(&v * 0.5, Vector::new(0.5, 1.0, 1.5));
        }

        #[test]
        fn division_by_a_scalar() {
            let v = Vector::new(1.0, 2.0, 3.0);
            assert_eq!(&v / 2.0, Vector::new(0.5, 1.0, 1.5));
        }
    }

    mod magnitude {
        use super::*;

        #[test]
        fn vector_1_0_0() {
            assert_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        }

        #[test]
        fn vector_0_1_0() {
            assert_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        }

        #[test]
        fn vector_0_0_1() {
            assert_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        }

        #[test]
        fn vector_1_2_3() {
            assert_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), 14.0_f64.sqrt());
        }

        #[test]
        fn vector_neg_1_neg_2_neg_3() {
            assert_eq!(Vector::new(-1.0, -2.0, -3.0).magnitude(), 14.0_f64.sqrt());
        }
    }

    mod normalization {
        use super::*;

        #[test]
        fn vector_1_2_3() {
            assert_eq!(
                Vector::new(1.0, 2.0, 3.0).normalize(),
                Vector::new(0.26726, 0.53452, 0.80178)
            );
        }

        #[test]
        fn normalized_vector_1_2_3() {
            assert_eq!(Vector::new(1.0, 2.0, 3.0).normalize().magnitude(), 1.0);
        }
    }

    mod product {
        use super::*;

        #[test]
        fn dot_product() {
            assert_eq!(
                Vector::new(1.0, 2.0, 3.0).dot(&Vector::new(2.0, 3.0, 4.0)),
                20.0
            );
        }

        #[test]
        fn cross_product() {
            assert_eq!(
                Vector::new(1.0, 2.0, 3.0).cross(&Vector::new(2.0, 3.0, 4.0)),
                Vector::new(-1.0, 2.0, -1.0)
            );
            assert_eq!(
                Vector::new(2.0, 3.0, 4.0).cross(&Vector::new(1.0, 2.0, 3.0)),
                Vector::new(1.0, -2.0, 1.0)
            );
        }
    }

    mod reflection {
        use super::*;

        #[test]
        fn reflection_approaching_at_45_degrees() {
            let r = Vector::new(1.0, -1.0, 0.0).reflect(&Vector::new(0.0, 1.0, 0.0));
            assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
        }

        #[test]
        fn reflection_off_a_slanted_surface() {
            let r = Vector::new(0.0, -1.0, 0.0).reflect(&Vector::new(
                2.0_f64.sqrt() / 2.0,
                2.0_f64.sqrt() / 2.0,
                0.0,
            ));
            assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
        }
    }
}
