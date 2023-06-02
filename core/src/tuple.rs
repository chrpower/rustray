#[derive(Debug, Copy, Clone)]
pub(crate) struct Tuple<const N: usize>([f64; N]);

impl<const N: usize> Tuple<N> {
    pub(crate) fn new(values: [f64; N]) -> Self {
        Tuple(values)
    }

    pub(crate) fn values(&self) -> &[f64; N] {
        &self.0
    }
}

use std::ops::{Add, Mul, Sub};
macro_rules! implement_operations {
    ($($trait:ident, $method:ident),+) => {
        $(
            impl<'a, 'b, const N: usize> $trait<&'b Tuple<N>> for &'a Tuple<N> {
                type Output = Tuple<N>;

                fn $method(self, other: &'b Self::Output) -> Self::Output {
                    let mut result: [f64; N] = [0.0; N];
                    for (i, item) in result.iter_mut().enumerate().take(N) {
                        *item = self.0[i].$method(other.0[i]);
                    }
                    Tuple(result)
                }
            }
        )+
    };
}
implement_operations!(Add, add);
implement_operations!(Sub, sub);
implement_operations!(Mul, mul);

impl<'a, const N: usize> Mul<f64> for &'a Tuple<N> {
    type Output = Tuple<N>;

    fn mul(self, scalar: f64) -> Self::Output {
        let mut result: [f64; N] = [0.0; N];
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = self.0[i] * scalar;
        }
        Tuple(result)
    }
}

use std::ops::Div;
impl<'a, const N: usize> Div<f64> for &'a Tuple<N> {
    type Output = Tuple<N>;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0 / scalar)
    }
}

use std::ops::Neg;
impl<'a, const N: usize> Neg for &'a Tuple<N> {
    type Output = Tuple<N>;

    fn neg(self) -> Self::Output {
        let mut result = [0.0; N];
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = -self.0[i];
        }
        Tuple(result)
    }
}

use std::cmp::PartialEq;
const EPSILON: f64 = 0.00001;
impl<const N: usize> PartialEq for Tuple<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(a, b)| (a - b).abs() < EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    mod creation {
        use super::*;

        #[test]
        fn access() {
            let t = Tuple::new([1.0, 2.0, 3.0, 4.0]);
            assert_eq!(t.values()[0], 1.0);
            assert_eq!(t.values()[1], 2.0);
            assert_eq!(t.values()[2], 3.0);
            assert_eq!(t.values()[3], 4.0);
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn identical() {
            assert_eq!(
                Tuple::new([1.0, 2.0, 3.0, 4.0]),
                Tuple::new([1.0, 2.0, 3.0, 4.0])
            );
        }

        #[test]
        fn small_difference() {
            assert_eq!(
                Tuple::new([1.0, 2.0, 3.0, 4.0]),
                Tuple::new([1.0, 2.0, 3.0, 4.00001])
            );
        }

        #[test]
        fn different() {
            assert_ne!(
                Tuple::new([1.0, 2.0, 3.0, 4.0]),
                Tuple::new([2.0, 3.0, 4.0, 5.0])
            );
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn addition() {
            assert_eq!(
                &Tuple::new([1.0, 2.0, 3.0, 4.0]) + &Tuple::new([2.0, 3.0, 4.0, 5.0]),
                Tuple::new([3.0, 5.0, 7.0, 9.0])
            );
        }

        #[test]
        fn subtraction() {
            assert_eq!(
                &Tuple::new([1.0, 2.0, 3.0, 4.0]) - &Tuple::new([2.0, 3.0, 4.0, 5.0]),
                Tuple::new([-1.0, -1.0, -1.0, -1.0])
            );
        }

        #[test]
        fn negation() {
            assert_eq!(
                -&Tuple::new([1.0, 2.0, 3.0, 4.0]),
                Tuple::new([-1.0, -2.0, -3.0, -4.0])
            );
        }

        #[test]
        fn multiplication() {
            assert_eq!(
                &Tuple::new([1.0, 2.0, 3.0, 4.0]) * &Tuple::new([2.0, 3.0, 4.0, 5.0]),
                Tuple::new([2.0, 6.0, 12.0, 20.0])
            );
        }

        #[test]
        fn multiplication_by_a_scalar() {
            assert_eq!(
                &Tuple::new([1.0, 2.0, 3.0, 4.0]) * 3.5,
                Tuple::new([3.5, 7.0, 10.5, 14.0])
            );
        }

        #[test]
        fn multiplication_by_a_fraction() {
            assert_eq!(
                &Tuple::new([1.0, 2.0, 3.0, 4.0]) * 0.5,
                Tuple::new([0.5, 1.0, 1.5, 2.0])
            );
        }

        #[test]
        fn division_by_a_scalar() {
            assert_eq!(
                &Tuple::new([1.0, 2.0, 3.0, 4.0]) / 2.0,
                Tuple::new([0.5, 1.0, 1.5, 2.0])
            );
        }
    }
}
