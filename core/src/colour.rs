use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    tuple: Tuple<3>,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Colour {
        Colour {
            tuple: Tuple::new([red, green, blue]),
        }
    }

    pub fn red(&self) -> f64 {
        self.tuple.values()[0]
    }

    pub fn green(&self) -> f64 {
        self.tuple.values()[1]
    }

    pub fn blue(&self) -> f64 {
        self.tuple.values()[2]
    }
}

use std::ops::{Add, Mul, Sub};
macro_rules! impl_colour_operator {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<'a, 'b> $trait<&'b Colour> for &'a Colour {
            type Output = Colour;

            fn $method(self, other: &'b Colour) -> Colour {
                let result = &self.tuple $op &other.tuple;
                Colour::new(result.values()[0], result.values()[1], result.values()[2])
            }
        }
    };
}

impl_colour_operator!(Add, add, +);
impl_colour_operator!(Sub, sub, -);
impl_colour_operator!(Mul, mul, *);

impl<'a> Mul<f64> for &'a Colour {
    type Output = Colour;

    fn mul(self, scalar: f64) -> Colour {
        let result = &self.tuple * scalar;
        Colour::new(result.values()[0], result.values()[1], result.values()[2])
    }
}

use std::cmp::PartialEq;
impl PartialEq for Colour {
    fn eq(&self, other: &Colour) -> bool {
        self.tuple == other.tuple
    }
}
#[cfg(test)]
mod test {
    use crate::Colour;

    mod creation {
        use super::*;

        #[test]
        fn access() {
            let c = Colour::new(-0.5, 0.4, 1.7);
            assert_eq!(c.red(), -0.5);
            assert_eq!(c.green(), 0.4);
            assert_eq!(c.blue(), 1.7);
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn identical() {
            assert_eq!(Colour::new(1.0, 2.0, 3.0), Colour::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn small_difference() {
            assert_eq!(Colour::new(1.0, 2.0, 3.0), Colour::new(1.0, 2.0, 3.000001));
        }

        #[test]
        fn different() {
            assert_ne!(Colour::new(1.0, 2.0, 3.0), Colour::new(2.0, 3.0, 4.0));
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn addition() {
            let c1 = Colour::new(0.9, 0.6, 0.75);
            let c2 = Colour::new(0.7, 0.1, 0.25);
            assert_eq!(&c1 + &c2, Colour::new(1.6, 0.7, 1.0));
        }

        #[test]
        fn subtraction() {
            let c1 = Colour::new(0.9, 0.6, 0.75);
            let c2 = Colour::new(0.7, 0.1, 0.25);
            assert_eq!(&c1 - &c2, Colour::new(0.2, 0.5, 0.5));
        }

        #[test]
        fn multiplication() {
            let c1 = Colour::new(1.0, 0.2, 0.4);
            let c2 = Colour::new(0.9, 1.0, 0.1);
            assert_eq!(&c1 * &c2, Colour::new(0.9, 0.2, 0.04));
        }

        #[test]
        fn multiplication_by_a_scalar() {
            let c = Colour::new(0.2, 0.3, 0.4);
            assert_eq!(&c * 2.0, Colour::new(0.4, 0.6, 0.8));
        }
    }
}
