use crate::square_matrix::SquareMatrix;
use core::Point;
use core::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    data: SquareMatrix<4>,
}

impl Matrix4 {
    pub fn new(matrix: [[f64; 4]; 4]) -> Self {
        Self {
            data: SquareMatrix::new(matrix),
        }
    }

    pub fn identity() -> Self {
        Self::from(SquareMatrix::identity())
    }

    pub fn transpose(&self) -> Self {
        Self::from(self.data.transpose())
    }

    pub fn inverse(&self) -> Self {
        Self::from(self.data.inverse().unwrap())
    }
}

use std::convert::From;
impl From<SquareMatrix<4>> for Matrix4 {
    fn from(square_matrix: SquareMatrix<4>) -> Self {
        Matrix4 {
            data: square_matrix,
        }
    }
}

use std::ops::Index;
impl Index<usize> for Matrix4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

use std::ops::IndexMut;
impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 4] {
        &mut self.data[index]
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

use std::ops::Mul;
macro_rules! mul_matrix {
    ($type:ty, $output:ty) => {
        impl<'a, 'b> Mul<&'b $type> for &'a Matrix4 {
            type Output = $output;

            fn mul(self, other: &'b $type) -> $output {
                &self.data * other
            }
        }
    };
}

mul_matrix!(Point, Point);
mul_matrix!(Vector, Vector);

impl<'a, 'b> Mul<&'b Matrix4> for &'a Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: &'b Matrix4) -> Matrix4 {
        Matrix4::from(&self.data * &other.data)
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix4;

    mod construction {
        use super::*;

        #[test]
        fn access() {
            let m = Matrix4::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);
            assert_eq!(m[0][0], 1.0);
            assert_eq!(m[1][1], 6.0);
            assert_eq!(m[2][2], 11.0);
            assert_eq!(m[3][3], 16.0);
        }

        #[test]
        fn access_and_update() {
            let mut m = Matrix4::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);
            m[0][0] = 5.5;
            assert_eq!(m[0][0], 5.5);
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn equality() {
            assert_eq!(
                Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]),
                Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
            );
        }

        #[test]
        fn small_difference() {
            assert_eq!(
                Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]),
                Matrix4::new([
                    [1.000001, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
            );
        }
        #[test]
        fn different() {
            assert_ne!(
                Matrix4::new([
                    [99.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]),
                Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
            );
        }
    }

    mod arithmetic {
        use super::*;
        use core::{Point, Vector};

        #[test]
        fn multiplication() {
            assert_eq!(
                &Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]) * &Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]),
                Matrix4::new([
                    [90.0, 100.0, 110.0, 120.0],
                    [202.0, 228.0, 254.0, 280.0],
                    [314.0, 356.0, 398.0, 440.0],
                    [426.0, 484.0, 542.0, 600.0]
                ])
            );
        }

        #[test]
        fn multiplication_vector() {
            assert_eq!(
                &Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [2.0, 4.0, 4.0, 2.0],
                    [8.0, 6.0, 4.0, 1.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]) * &Vector::new(1.0, 2.0, 3.0),
                Vector::new(14.0, 22.0, 32.0)
            );
        }

        #[test]
        fn multiplication_point() {
            assert_eq!(
                &Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [2.0, 4.0, 4.0, 2.0],
                    [8.0, 6.0, 4.0, 1.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]) * &Point::new(1.0, 2.0, 3.0),
                Point::new(18.0, 24.0, 33.0)
            );
        }
    }

    mod identity {
        use super::*;

        #[test]
        fn identity() {
            assert_eq!(
                Matrix4::identity(),
                Matrix4::new([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ])
            );
        }

        #[test]
        fn mul_identity() {
            assert_eq!(
                &Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]) * &Matrix4::identity(),
                Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
            );
        }
    }

    mod transposition {
        use super::*;

        #[test]
        fn transpose() {
            assert_eq!(
                Matrix4::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
                .transpose(),
                Matrix4::new([
                    [1.0, 5.0, 9.0, 13.0],
                    [2.0, 6.0, 10.0, 14.0],
                    [3.0, 7.0, 11.0, 15.0],
                    [4.0, 8.0, 12.0, 16.0]
                ])
            );
        }

        #[test]
        fn identity_transpose() {
            assert_eq!(Matrix4::identity().transpose(), Matrix4::identity());
        }
    }

    mod inverse {
        use super::*;

        #[test]
        fn inverse() {
            assert_eq!(
                Matrix4::new([
                    [-5.0, 2.0, 6.0, -8.0],
                    [1.0, -5.0, 1.0, 8.0],
                    [7.0, 7.0, -6.0, -7.0],
                    [1.0, -3.0, 7.0, 4.0]
                ])
                .inverse(),
                Matrix4::new([
                    [0.21805, 0.45113, 0.24060, -0.04511],
                    [-0.80827, -1.45677, -0.44361, 0.52068],
                    [-0.07895, -0.22368, -0.05263, 0.19737],
                    [-0.52256, -0.81391, -0.30075, 0.30639]
                ])
            );
        }

        #[test]
        #[should_panic]
        fn cannot_be_inverted() {
            let _ = Matrix4::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ])
            .inverse();
        }
    }
}
