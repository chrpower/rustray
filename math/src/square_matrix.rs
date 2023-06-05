#[derive(Debug, Clone, Copy)]
pub(crate) struct SquareMatrix<const N: usize> {
    pub(crate) data: [[f64; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
    pub(crate) fn new(data: [[f64; N]; N]) -> Self {
        Self { data }
    }

    fn zeros() -> Self {
        SquareMatrix::new([[0.0; N]; N])
    }

    pub(crate) fn identity() -> Self {
        let mut result = SquareMatrix::new([[0.0; N]; N]);
        for i in 0..N {
            result[i][i] = 1.0;
        }
        result
    }

    pub(crate) fn transpose(&self) -> Self {
        let mut result = SquareMatrix::new([[0.0; N]; N]);
        for (i, row) in self.data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                result[j][i] = *val;
            }
        }
        result
    }
}

impl SquareMatrix<4> {
    fn submatrix(&self, row: usize, col: usize) -> SquareMatrix<3> {
        if row >= 4 || col >= 4 {
            panic!("Invalid indices for a 4x4 matrix : {}, {}", row, col);
        }

        let mut result = SquareMatrix::<3>::zeros();
        let mut row_index = 0;
        for (i, row_val) in self.data.iter().enumerate() {
            if i == row {
                continue;
            }
            let mut col_index = 0;
            for (j, _) in row_val.iter().enumerate() {
                if j == col {
                    continue;
                }
                result[row_index][col_index] = self[i][j];
                col_index += 1;
            }
            row_index += 1;
        }
        result
    }

    fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for (col, _) in self.data[0].iter().enumerate() {
            result += self[0][col] * self.cofactor(0, col);
        }
        result
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub(crate) fn inverse(&self) -> Result<Self, &'static str> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return Err("Cannot invert matrix with determinant of 0");
        }
        let mut result = SquareMatrix::new([[0.0; 4]; 4]);
        for (row, row_val) in self.data.iter().enumerate() {
            for (col, _) in row_val.iter().enumerate() {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }
        Ok(result)
    }
}

impl SquareMatrix<3> {
    fn submatrix(&self, row: usize, col: usize) -> SquareMatrix<2> {
        if row >= 3 || col >= 3 {
            panic!("Invalid indices for a 3x3 matrix: {}, {}", row, col);
        }

        let mut result = SquareMatrix::<2>::zeros();
        let mut row_index = 0;
        for (i, row_val) in self.data.iter().enumerate() {
            if i == row {
                continue;
            }
            let mut col_index = 0;
            for (j, _) in row_val.iter().enumerate() {
                if j == col {
                    continue;
                }
                result[row_index][col_index] = self[i][j];
                col_index += 1;
            }
            row_index += 1;
        }
        result
    }

    fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for (col, _) in self.data[0].iter().enumerate() {
            result += self[0][col] * self.cofactor(0, col);
        }
        result
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    #[allow(dead_code)]
    fn inverse(&self) -> Result<SquareMatrix<3>, &'static str> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return Err("Cannot invert matrix with determinant of 0");
        }
        let mut result = SquareMatrix::new([[0.0; 3]; 3]);
        for (row, row_val) in self.data.iter().enumerate() {
            for (col, _) in row_val.iter().enumerate() {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }
        Ok(result)
    }
}

impl SquareMatrix<2> {
    #[allow(dead_code)]
    fn submatrix(&self, row: usize, col: usize) -> SquareMatrix<1> {
        let value = match (row, col) {
            (0, 0) => self.data[1][1],
            (0, 1) => self.data[1][0],
            (1, 0) => self.data[0][1],
            (1, 1) => self.data[0][0],
            _ => panic!("Invalid indices for a 2x2 matrix: {}, {}", row, col),
        };

        SquareMatrix::new([[value]])
    }

    fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        match (row, col) {
            (0, 0) => self[1][1],
            (0, 1) => self[1][0],
            (1, 0) => self[0][1],
            (1, 1) => self[0][0],
            _ => panic!("Invalid row or column index for 2x2 matrix"),
        }
    }

    #[allow(dead_code)]
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    #[allow(dead_code)]
    fn inverse(&self) -> Result<SquareMatrix<2>, &'static str> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return Err("Cannot invert matrix with determinant of 0");
        }
        let mut result = SquareMatrix::new([[0.0; 2]; 2]);
        for (row, row_val) in self.data.iter().enumerate() {
            for (col, _) in row_val.iter().enumerate() {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }
        Ok(result)
    }
}

use std::ops::Index;
impl<const N: usize> Index<usize> for SquareMatrix<N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

use std::ops::IndexMut;
impl<const N: usize> IndexMut<usize> for SquareMatrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut [f64; N] {
        &mut self.data[index]
    }
}

const EPSILON: f64 = 0.00001;
impl<const N: usize> PartialEq for SquareMatrix<N> {
    fn eq(&self, other: &Self) -> bool {
        let flat_self = self.data.iter().flatten();
        let flat_other = other.data.iter().flatten();
        flat_self
            .zip(flat_other)
            .all(|(a, b)| (a - b).abs() < EPSILON)
    }
}

use std::ops::Mul;
impl<'a, 'b, const N: usize> Mul<&'b SquareMatrix<N>> for &'a SquareMatrix<N> {
    type Output = SquareMatrix<N>;

    fn mul(self, other: &'b SquareMatrix<N>) -> SquareMatrix<N> {
        let mut result = SquareMatrix::new([[0.0; N]; N]);
        for (i, self_row) in self.data.iter().enumerate() {
            for (j, _) in other.data.iter().enumerate() {
                for (k, self_val) in self_row.iter().enumerate() {
                    result[i][j] += self_val * other.data[k][j];
                }
            }
        }
        result
    }
}

macro_rules! impl_mul {
    ($($t:ty)*) => ($(
        impl<'a, 'b> Mul<&'b $t> for &'a SquareMatrix<4> {
            type Output = $t;

            fn mul(self, other: &'b $t) -> $t {
                let x = self[0][0] * other.x()
                    + self[0][1] * other.y()
                    + self[0][2] * other.z()
                    + self[0][3] * other.w();
                let y = self[1][0] * other.x()
                    + self[1][1] * other.y()
                    + self[1][2] * other.z()
                    + self[1][3] * other.w();
                let z = self[2][0] * other.x()
                    + self[2][1] * other.y()
                    + self[2][2] * other.z()
                    + self[2][3] * other.w();
                <$t>::new(x, y, z)
            }
        }
    )*)
}

impl_mul! {
    core::Point
    core::Vector
}

#[cfg(test)]
mod test {
    use crate::square_matrix::SquareMatrix;

    mod construction {
        use super::*;

        #[test]
        fn matrix2x2_access() {
            let m = SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]);
            assert_eq!(m[0][0], 1.0);
            assert_eq!(m[1][1], 6.0);
        }

        #[test]
        fn matrix3x3_access() {
            let m = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);
            assert_eq!(m[0][0], 1.0);
            assert_eq!(m[1][1], 6.0);
            assert_eq!(m[2][2], 11.0);
        }

        #[test]
        fn matrix4x4_access() {
            let m = SquareMatrix::new([
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
            let mut m = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);
            m[0][0] = 5.5;
            assert_eq!(m[0][0], 5.5);
        }
    }

    mod equality {
        use super::*;

        #[test]
        fn equal() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]),
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]])
            );
        }

        #[test]
        fn small_difference() {
            assert_eq!(
                SquareMatrix::new([[1.0000001, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]),
                SquareMatrix::new([[1.0, 2.0000001, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
            );
        }
        #[test]
        fn different() {
            assert_ne!(
                SquareMatrix::new([
                    [99.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]),
                SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
            );
        }
    }

    mod multiplication {
        use core::{Point, Vector};

        use super::*;

        #[test]
        fn matrix2x2() {
            assert_eq!(
                &SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]])
                    * &SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]),
                SquareMatrix::new([[11.0, 14.0], [35.0, 46.0]])
            );
        }

        #[test]
        fn matrix3x3() {
            assert_eq!(
                &SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    * &SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]),
                SquareMatrix::new([
                    [38.0, 44.0, 50.0],
                    [98.0, 116.0, 134.0],
                    [158.0, 188.0, 218.0]
                ])
            );
        }

        #[test]
        fn matrix4x4() {
            assert_eq!(
                &SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]) * &SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]),
                SquareMatrix::new([
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
                &SquareMatrix::new([
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
                &SquareMatrix::new([
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
        fn matrix2x2() {
            assert_eq!(
                SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 0.0], [0.0, 1.0]])
            );
        }

        #[test]
        fn matrix2x2_multiplication() {
            assert_eq!(
                &SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]) * &SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]])
            );
        }

        #[test]
        fn matrix3x3() {
            assert_eq!(
                SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
            );
        }

        #[test]
        fn matrix3x3_mul() {
            assert_eq!(
                &SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    * &SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
            );
        }

        #[test]
        fn matrix4x4() {
            assert_eq!(
                SquareMatrix::identity(),
                SquareMatrix::new([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ])
            );
        }

        #[test]
        fn matrix4x4_multiplication() {
            assert_eq!(
                &SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]) * &SquareMatrix::identity(),
                SquareMatrix::new([
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
        fn matrix2x2() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]).transpose(),
                SquareMatrix::new([[1.0, 5.0], [2.0, 6.0]])
            );
        }

        #[test]
        fn matrix3x3() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    .transpose(),
                SquareMatrix::new([[1.0, 5.0, 9.0], [2.0, 6.0, 10.0], [3.0, 7.0, 11.0]])
            );
        }

        #[test]
        fn matrix4x4() {
            assert_eq!(
                SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
                .transpose(),
                SquareMatrix::new([
                    [1.0, 5.0, 9.0, 13.0],
                    [2.0, 6.0, 10.0, 14.0],
                    [3.0, 7.0, 11.0, 15.0],
                    [4.0, 8.0, 12.0, 16.0]
                ])
            );
        }

        #[test]
        fn matrix4x4_identity_transposition() {
            assert_eq!(
                SquareMatrix::<4>::identity().transpose(),
                SquareMatrix::<4>::identity()
            );
        }
    }

    mod submatrix {
        use super::*;

        #[test]
        fn matrix2x2_00() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]).submatrix(0, 0),
                SquareMatrix::new([[6.0]])
            );
        }

        #[test]
        fn matrix3x3_11() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    .submatrix(1, 1),
                SquareMatrix::new([[1.0, 3.0], [9.0, 11.0]])
            );
        }

        #[test]
        fn matrix4x4_11() {
            assert_eq!(
                SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
                .submatrix(1, 1),
                SquareMatrix::new([[1.0, 3.0, 4.0], [9.0, 11.0, 12.0], [13.0, 15.0, 16.0]])
            );
        }

        #[test]
        fn matrix4x4_22() {
            assert_eq!(
                SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
                .submatrix(2, 2),
                SquareMatrix::new([[1.0, 2.0, 4.0], [5.0, 6.0, 8.0], [13.0, 14.0, 16.0]])
            );
        }
    }

    mod minor {
        use super::*;

        #[test]
        fn matrix2x2() {
            let matrix = SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]);

            let mut calculated_minors = SquareMatrix::zeros();
            for row in 0..2 {
                for col in 0..2 {
                    calculated_minors[row][col] = matrix.minor(row, col);
                }
            }

            assert_eq!(
                calculated_minors,
                SquareMatrix::new([[6.0, 5.0], [2.0, 1.0]])
            );
        }

        #[test]
        fn matrix3x3() {
            let matrix = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);

            let mut calculated_minors = SquareMatrix::zeros();
            for row in 0..3 {
                for col in 0..3 {
                    calculated_minors[row][col] = matrix.minor(row, col);
                }
            }

            assert_eq!(
                calculated_minors,
                SquareMatrix::new([[-4.0, -8.0, -4.0], [-8.0, -16.0, -8.0], [-4.0, -8.0, -4.0]])
            );
        }

        #[test]
        fn matrix4x4() {
            let matrix = SquareMatrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);

            let mut calculated_minors = SquareMatrix::zeros();
            for row in 0..4 {
                for col in 0..4 {
                    calculated_minors[row][col] = matrix.minor(row, col);
                }
            }

            assert_eq!(
                calculated_minors,
                SquareMatrix::new([
                    [0.0, 6.661338147750939e-16, 1.3322676295501878e-15, 0.0],
                    [
                        0.0,
                        3.9968028886505635e-15,
                        7.993605777301127e-15,
                        1.4210854715202004e-14,
                    ],
                    [
                        0.0,
                        1.9984014443252818e-15,
                        3.9968028886505635e-15,
                        7.105427357601002e-15,
                    ],
                    [0.0, 1.3322676295501878e-15, 2.6645352591003757e-15, 0.0],
                ])
            );
        }
    }

    mod cofactors {
        use super::*;

        #[test]
        fn matrix2x2() {
            let matrix = SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]);

            let mut calculated_cofactors = SquareMatrix::zeros();
            for row in 0..2 {
                for col in 0..2 {
                    calculated_cofactors[row][col] = matrix.cofactor(row, col);
                }
            }
            assert_eq!(
                calculated_cofactors,
                SquareMatrix::new([[6.0, -5.0], [-2.0, 1.0]])
            );
        }

        #[test]
        fn matrix3x3() {
            let matrix = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);

            let mut calculated_cofactors = SquareMatrix::zeros();
            for row in 0..3 {
                for col in 0..3 {
                    calculated_cofactors[row][col] = matrix.cofactor(row, col);
                }
            }

            assert_eq!(
                calculated_cofactors,
                SquareMatrix::new([[-4.0, 8.0, -4.0], [8.0, -16.0, 8.0], [-4.0, 8.0, -4.0]])
            );
        }

        #[test]
        fn matrix4x4() {
            let matrix = SquareMatrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);

            let mut calculated_cofactors = SquareMatrix::zeros();
            for row in 0..4 {
                for col in 0..4 {
                    calculated_cofactors[row][col] = matrix.cofactor(row, col);
                }
            }

            assert_eq!(
                calculated_cofactors,
                SquareMatrix::new([
                    [0.0, -6.661338147750939e-16, 1.3322676295501878e-15, 0.0],
                    [
                        0.0,
                        3.9968028886505635e-15,
                        -7.993605777301127e-15,
                        1.4210854715202004e-14,
                    ],
                    [
                        0.0,
                        -1.9984014443252818e-15,
                        3.9968028886505635e-15,
                        -7.105427357601002e-15,
                    ],
                    [0.0, 1.3322676295501878e-15, -2.6645352591003757e-15, 0.0],
                ])
            );
        }
    }

    mod determinant {
        use super::*;

        #[test]
        fn matrix2x2() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]).determinant(),
                -4.0
            );
        }

        #[test]
        fn matrix3x3() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    .determinant(),
                0.0
            );
        }

        #[test]
        fn matrix4x4() {
            assert_eq!(
                SquareMatrix::new([
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
                .determinant(),
                0.0
            );
        }
    }

    mod inverse {
        use super::*;

        #[test]
        fn matrix2x2() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]])
                    .inverse()
                    .unwrap(),
                SquareMatrix::new([[-1.5, 0.5], [1.25, -0.25]])
            );
        }

        #[test]
        fn matrix3x3() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 8.0], [9.0, 10.0, 12.0]])
                    .inverse()
                    .unwrap(),
                SquareMatrix::new([[-2.0, 1.5, -0.5], [3.0, -3.75, 1.75], [-1.0, 2.0, -1.0]])
            );
        }

        #[test]
        fn matrix4x4() {
            assert_eq!(
                SquareMatrix::new([
                    [-5.0, 2.0, 6.0, -8.0],
                    [1.0, -5.0, 1.0, 8.0],
                    [7.0, 7.0, -6.0, -7.0],
                    [1.0, -3.0, 7.0, 4.0]
                ])
                .inverse()
                .unwrap(),
                SquareMatrix::new([
                    [0.21805, 0.45113, 0.24060, -0.04511],
                    [-0.80827, -1.45677, -0.44361, 0.52068],
                    [-0.07895, -0.22368, -0.05263, 0.19737],
                    [-0.52256, -0.81391, -0.30075, 0.30639]
                ])
            );
        }

        #[test]
        fn cannot_be_inverted() {
            assert!(SquareMatrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ])
            .inverse()
            .is_err());
        }
    }
}
