#[derive(Debug, Clone, Copy)]
pub struct SquareMatrix<const N: usize> {
    data: [[f64; N]; N],
}

impl<const N: usize> SquareMatrix<N> {
    pub fn new(data: [[f64; N]; N]) -> Self {
        Self { data }
    }

    pub fn zeros() -> SquareMatrix<N> {
        SquareMatrix::new([[0.0; N]; N])
    }

    pub fn identity() -> SquareMatrix<N> {
        let mut result = SquareMatrix::new([[0.0; N]; N]);
        for i in 0..N {
            result[i][i] = 1.0;
        }
        result
    }

    pub fn transpose(&self) -> SquareMatrix<N> {
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
    pub fn submatrix(&self, row: usize, col: usize) -> SquareMatrix<3> {
        if row >= 4 || col >= 4 {
            panic!("Invalid indices: {}, {}", row, col);
        }

        let mut result = [[0.0; 3]; 3];
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
        SquareMatrix::<3>::new(result)
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for (col, _) in self.data[0].iter().enumerate() {
            result += self[0][col] * self.cofactor(0, col);
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn inverse(&self) -> SquareMatrix<4> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            panic!("Cannot invert matrix with determinant of 0");
        }
        let mut result = SquareMatrix::new([[0.0; 4]; 4]);
        for (row, row_val) in self.data.iter().enumerate() {
            for (col, _) in row_val.iter().enumerate() {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }
        result
    }
}

impl SquareMatrix<3> {
    pub fn submatrix(&self, row: usize, col: usize) -> SquareMatrix<2> {
        if row >= 3 || col >= 3 {
            panic!("Invalid indices: {}, {}", row, col);
        }

        let mut result = [[0.0; 2]; 2];
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
        SquareMatrix::<2>::new(result)
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for (col, _) in self.data[0].iter().enumerate() {
            result += self[0][col] * self.cofactor(0, col);
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn inverse(&self) -> SquareMatrix<3> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            panic!("Cannot invert matrix with determinant of 0");
        }
        let mut result = SquareMatrix::new([[0.0; 3]; 3]);
        for (row, row_val) in self.data.iter().enumerate() {
            for (col, _) in row_val.iter().enumerate() {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }
        result
    }
}

impl SquareMatrix<2> {
    pub fn submatrix(&self, row: usize, col: usize) -> SquareMatrix<1> {
        let value = match (row, col) {
            (0, 0) => self.data[1][1],
            (0, 1) => self.data[1][0],
            (1, 0) => self.data[0][1],
            (1, 1) => self.data[0][0],
            _ => panic!("Invalid indices: {}, {}", row, col),
        };

        SquareMatrix::new([[value]])
    }

    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        match (row, col) {
            (0, 0) => self[1][1],
            (0, 1) => self[1][0],
            (1, 0) => self[0][1],
            (1, 1) => self[0][0],
            _ => panic!("Invalid row or column index for 2x2 matrix"),
        }
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn inverse(&self) -> SquareMatrix<2> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            panic!("Cannot invert matrix with determinant of 0");
        }
        let mut result = SquareMatrix::new([[0.0; 2]; 2]);
        for (row, row_val) in self.data.iter().enumerate() {
            for (col, _) in row_val.iter().enumerate() {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }
        result
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

use core::Point;
impl<'a, 'b> Mul<&'b Point> for &'a SquareMatrix<4> {
    type Output = Point;

    fn mul(self, other: &'b Point) -> Point {
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
        Point::new(x, y, z)
    }
}

use core::Vector;
impl<'a, 'b> Mul<&'b Vector> for &'a SquareMatrix<4> {
    type Output = Vector;

    fn mul(self, other: &'b Vector) -> Vector {
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
        Vector::new(x, y, z)
    }
}

#[cfg(test)]
mod test {

    mod construction {
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix2x2_can_be_represented_and_accessed() {
            let m = SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]);
            assert_eq!(m[0][0], 1.0);
            assert_eq!(m[1][1], 6.0);
        }

        #[test]
        fn matrix3x3_can_be_represented_and_accessed() {
            let m = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);
            assert_eq!(m[0][0], 1.0);
            assert_eq!(m[1][1], 6.0);
            assert_eq!(m[2][2], 11.0);
        }

        #[test]
        fn matrix4x4_can_be_represented_and_accessed() {
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
        fn mut_matrix_can_be_updated() {
            let mut m = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);
            m[0][0] = 5.5;
            assert_eq!(m[0][0], 5.5);
        }
    }

    mod equality {
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix_equality() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]),
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]])
            );
        }

        #[test]
        fn matrix_equality_with_small_difference() {
            assert_eq!(
                SquareMatrix::new([[1.0000001, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]),
                SquareMatrix::new([[1.0, 2.0000001, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
            );
        }
        #[test]
        fn matrix_inequality() {
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
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix2x2_multiplication() {
            assert_eq!(
                &SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]])
                    * &SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]),
                SquareMatrix::new([[11.0, 14.0], [35.0, 46.0]])
            );
        }

        #[test]
        fn matrix3x3_multiplication() {
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
        fn matrix4x4_multiplication() {
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
    }

    mod identity {
        use crate::matrix::SquareMatrix;

        #[test]
        fn identity_matrix2x2() {
            assert_eq!(
                SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 0.0], [0.0, 1.0]])
            );
        }

        #[test]
        fn matrix2x2_mul_identity() {
            assert_eq!(
                &SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]) * &SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]])
            );
        }

        #[test]
        fn identity_matrix3x3() {
            assert_eq!(
                SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
            );
        }

        #[test]
        fn matrix3x3_mul_identity() {
            assert_eq!(
                &SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    * &SquareMatrix::identity(),
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
            );
        }

        #[test]
        fn identity_matrix4x4() {
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
        fn matrix4x4_mul_identity() {
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
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix2x2_transposition() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]).transpose(),
                SquareMatrix::new([[1.0, 5.0], [2.0, 6.0]])
            );
        }

        #[test]
        fn matrix3x3_transposition() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    .transpose(),
                SquareMatrix::new([[1.0, 5.0, 9.0], [2.0, 6.0, 10.0], [3.0, 7.0, 11.0]])
            );
        }

        #[test]
        fn matrix4x4_transposition() {
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
        fn identity_matrix4x4_transposition() {
            assert_eq!(
                SquareMatrix::<4>::identity().transpose(),
                SquareMatrix::<4>::identity()
            );
        }
    }

    mod submatrix {
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix2x2_submatrix_00() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]).submatrix(0, 0),
                SquareMatrix::new([[6.0]])
            );
        }

        #[test]
        fn matrix3x3_submatrix_11() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    .submatrix(1, 1),
                SquareMatrix::new([[1.0, 3.0], [9.0, 11.0]])
            );
        }

        #[test]
        fn matrix4x4_submatrix_11() {
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
        fn matrix4x4_submatrix_22() {
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
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix2x2_minors() {
            let matrix = SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]);

            let mut calculated_minors = SquareMatrix::zeros();
            for row in 0..2 {
                for col in 0..2 {
                    calculated_minors[row][col] = matrix.minor(row, col);
                }
            }

            let expected_minors = SquareMatrix::new([[6.0, 5.0], [2.0, 1.0]]);
            assert_eq!(calculated_minors, expected_minors);
        }

        #[test]
        fn matrix3x3_minors() {
            let matrix = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);

            let mut calculated_minors = SquareMatrix::zeros();
            for row in 0..3 {
                for col in 0..3 {
                    calculated_minors[row][col] = matrix.minor(row, col);
                }
            }
            let expected_minors =
                SquareMatrix::new([[-4.0, -8.0, -4.0], [-8.0, -16.0, -8.0], [-4.0, -8.0, -4.0]]);

            assert_eq!(calculated_minors, expected_minors);
        }

        #[test]
        fn matrix4x4_minors() {
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

            let expected_minors = SquareMatrix::new([
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
            ]);
            assert_eq!(calculated_minors, expected_minors);
        }
    }

    mod cofactors {
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix2x2_cofactor() {
            let matrix = SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]);

            let mut calculated_cofactors = SquareMatrix::zeros();
            for row in 0..2 {
                for col in 0..2 {
                    calculated_cofactors[row][col] = matrix.cofactor(row, col);
                }
            }

            let expected_cofactors = SquareMatrix::new([[6.0, -5.0], [-2.0, 1.0]]);
            assert_eq!(calculated_cofactors, expected_cofactors);
        }

        #[test]
        fn matrix3x3_cofactor() {
            let matrix = SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]);

            let mut calculated_cofactors = SquareMatrix::zeros();
            for row in 0..3 {
                for col in 0..3 {
                    calculated_cofactors[row][col] = matrix.cofactor(row, col);
                }
            }

            let expected_cofactors =
                SquareMatrix::new([[-4.0, 8.0, -4.0], [8.0, -16.0, 8.0], [-4.0, 8.0, -4.0]]);
            assert_eq!(calculated_cofactors, expected_cofactors);
        }

        #[test]
        fn matrix4x4_cofactor() {
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

            let expected_cofactors = SquareMatrix::new([
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
            ]);
            assert_eq!(calculated_cofactors, expected_cofactors);
        }
    }

    mod determinant {
        use crate::SquareMatrix;

        #[test]
        fn matrix2x2_determinant() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]).determinant(),
                -4.0
            );
        }

        #[test]
        fn matrix3x3_determinant() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])
                    .determinant(),
                0.0
            );
        }

        #[test]
        fn matrix4x4_determinant() {
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
        use crate::matrix::SquareMatrix;

        #[test]
        fn matrix2x2_inverse() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0], [5.0, 6.0]]).inverse(),
                SquareMatrix::new([[-1.5, 0.5], [1.25, -0.25]])
            );
        }

        #[test]
        fn matrix3x3_inverse() {
            assert_eq!(
                SquareMatrix::new([[1.0, 2.0, 3.0], [5.0, 6.0, 8.0], [9.0, 10.0, 12.0]]).inverse(),
                SquareMatrix::new([[-2.0, 1.5, -0.5], [3.0, -3.75, 1.75], [-1.0, 2.0, -1.0]])
            );
        }

        #[test]
        fn matrix4x4_inverse() {
            assert_eq!(
                SquareMatrix::new([
                    [-5.0, 2.0, 6.0, -8.0],
                    [1.0, -5.0, 1.0, 8.0],
                    [7.0, 7.0, -6.0, -7.0],
                    [1.0, -3.0, 7.0, 4.0]
                ])
                .inverse(),
                SquareMatrix::new([
                    [0.21805, 0.45113, 0.24060, -0.04511],
                    [-0.80827, -1.45677, -0.44361, 0.52068],
                    [-0.07895, -0.22368, -0.05263, 0.19737],
                    [-0.52256, -0.81391, -0.30075, 0.30639]
                ])
            );
        }

        #[test]
        #[should_panic]
        fn matrix_cannot_be_inverted() {
            let _ = SquareMatrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ])
            .inverse();
        }
    }
}
