use crate::SquareMatrix;

pub fn translation(x: f64, y: f64, z: f64) -> SquareMatrix<4> {
    let mut matrix = SquareMatrix::identity();
    matrix[0][3] = x;
    matrix[1][3] = y;
    matrix[2][3] = z;
    matrix
}

pub fn scaling(x: f64, y: f64, z: f64) -> SquareMatrix<4> {
    let mut matrix = SquareMatrix::identity();
    matrix[0][0] = x;
    matrix[1][1] = y;
    matrix[2][2] = z;
    matrix
}

pub fn rotation_x(radians: f64) -> SquareMatrix<4> {
    let mut matrix = SquareMatrix::identity();
    matrix[1][1] = radians.cos();
    matrix[1][2] = -radians.sin();
    matrix[2][1] = radians.sin();
    matrix[2][2] = radians.cos();
    matrix
}

pub fn rotation_y(radians: f64) -> SquareMatrix<4> {
    let mut matrix = SquareMatrix::identity();
    matrix[0][0] = radians.cos();
    matrix[0][2] = radians.sin();
    matrix[2][0] = -radians.sin();
    matrix[2][2] = radians.cos();
    matrix
}

pub fn rotation_z(radians: f64) -> SquareMatrix<4> {
    let mut matrix = SquareMatrix::identity();
    matrix[0][0] = radians.cos();
    matrix[0][1] = -radians.sin();
    matrix[1][0] = radians.sin();
    matrix[1][1] = radians.cos();
    matrix
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> SquareMatrix<4> {
    let mut matrix = SquareMatrix::identity();
    matrix[0][1] = xy;
    matrix[0][2] = xz;
    matrix[1][0] = yx;
    matrix[1][2] = yz;
    matrix[2][0] = zx;
    matrix[2][1] = zy;
    matrix
}

#[cfg(test)]
mod test {
    use super::*;
    use core::{Point, Vector};

    #[test]
    fn test_translation() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &p, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_translation_inverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(&inv * &p, Point::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_translation_vector() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &v, v);
    }

    #[test]
    fn test_scaling_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &p, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scaling_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &v, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scaling_inverse() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(&inv * &v, Vector::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_scaling_reflection() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotation_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_x(std::f64::consts::PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(&full_quarter * &p, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotation_x_inverse() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let inv = half_quarter.inverse();
        assert_eq!(
            &inv * &p,
            Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn test_rotation_y() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_y(std::f64::consts::PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(&full_quarter * &p, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotation_z() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_z(std::f64::consts::PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(&full_quarter * &p, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing_x_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_x_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_y_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shearing_y_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_z_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_z_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_transform_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let p2 = &a * &p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));
        let p3 = &b * &p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0));
        let p4 = &c * &p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn test_transform_chaining() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = &c * &(&b * &a);
        assert_eq!(&t * &p, Point::new(15.0, 0.0, 7.0));
    }
}
