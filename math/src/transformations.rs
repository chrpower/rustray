use core::{Point, Vector};

use crate::Matrix4;

pub struct Transform {
    matrix: Matrix4,
}

impl Transform {
    fn new(matrix: Matrix4) -> Self {
        Self { matrix }
    }

    pub fn translation(mut self, x: f64, y: f64, z: f64) -> Self {
        let translation = Matrix4::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.matrix = &translation * &self.matrix;
        self
    }

    pub fn scaling(mut self, x: f64, y: f64, z: f64) -> Self {
        let scaling = Matrix4::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.matrix = &scaling * &self.matrix;
        self
    }

    pub fn rotation_x(mut self, radians: f64) -> Self {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let rotation = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_theta, -sin_theta, 0.0],
            [0.0, sin_theta, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.matrix = &rotation * &self.matrix;
        self
    }

    pub fn rotation_y(mut self, radians: f64) -> Self {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let rotation = Matrix4::new([
            [cos_theta, 0.0, sin_theta, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_theta, 0.0, cos_theta, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.matrix = &rotation * &self.matrix;
        self
    }

    pub fn rotation_z(mut self, radians: f64) -> Self {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let rotation = Matrix4::new([
            [cos_theta, -sin_theta, 0.0, 0.0],
            [sin_theta, cos_theta, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.matrix = &rotation * &self.matrix;
        self
    }

    pub fn shearing(mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let shearing = Matrix4::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        self.matrix = &shearing * &self.matrix;
        self
    }

    pub fn view_transform(mut self, from: Point, to: Point, up: Vector) -> Self {
        let forward = (&to - &from).normalize();
        let left = forward.cross(&up.normalize());
        let true_up = left.cross(&forward);

        let orientation = Matrix4::new([
            [left.x(), left.y(), left.z(), 0.0],
            [true_up.x(), true_up.y(), true_up.z(), 0.0],
            [-forward.x(), -forward.y(), -forward.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let transform = &orientation
            * &Transform::default()
                .translation(-from.x(), -from.y(), -from.z())
                .build();

        self.matrix = &transform * &self.matrix;
        self
    }

    pub fn build(self) -> Matrix4 {
        self.matrix
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Matrix4::identity())
    }
}

#[cfg(test)]
mod test {
    use crate::transformations::Transform;
    use core::{Point, Vector};

    mod translation {
        use super::*;

        #[test]
        fn point() {
            assert_eq!(
                &Transform::default().translation(5.0, -3.0, 2.0).build()
                    * &Point::new(-3.0, 4.0, 5.0),
                Point::new(2.0, 1.0, 7.0)
            );
        }

        #[test]
        fn inverse_point() {
            assert_eq!(
                &Transform::default()
                    .translation(5.0, -3.0, 2.0)
                    .build()
                    .inverse()
                    * &Point::new(-3.0, 4.0, 5.0),
                Point::new(-8.0, 7.0, 3.0)
            );
        }

        #[test]
        fn vector_unchanged() {
            let transform = Transform::default().translation(5.0, -3.0, 2.0).build();
            let v = Vector::new(-3.0, 4.0, 5.0);
            assert_eq!(&transform * &v, v);
        }
    }

    mod scaling {
        use super::*;

        #[test]
        fn point() {
            assert_eq!(
                &Transform::default().scaling(2.0, 3.0, 4.0).build() * &Point::new(-4.0, 6.0, 8.0),
                Point::new(-8.0, 18.0, 32.0)
            );
        }

        #[test]
        fn vector() {
            assert_eq!(
                &Transform::default().scaling(2.0, 3.0, 4.0).build() * &Vector::new(-4.0, 6.0, 8.0),
                Vector::new(-8.0, 18.0, 32.0)
            );
        }

        #[test]
        fn inverse_vector() {
            assert_eq!(
                &Transform::default()
                    .scaling(2.0, 3.0, 4.0)
                    .build()
                    .inverse()
                    * &Vector::new(-4.0, 6.0, 8.0),
                Vector::new(-2.0, 2.0, 2.0)
            );
        }

        #[test]
        fn reflection() {
            assert_eq!(
                &Transform::default().scaling(-1.0, 1.0, 1.0).build() * &Point::new(2.0, 3.0, 4.0),
                Point::new(-2.0, 3.0, 4.0)
            );
        }
    }

    mod rotation {
        use super::*;

        #[test]
        fn x() {
            let half_quarter = Transform::default()
                .rotation_x(std::f64::consts::PI / 4.0)
                .build();

            let full_quarter = Transform::default()
                .rotation_x(std::f64::consts::PI / 2.0)
                .build();

            let p = Point::new(0.0, 1.0, 0.0);

            assert_eq!(
                &half_quarter * &p,
                Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
            );
            assert_eq!(&full_quarter * &p, Point::new(0.0, 0.0, 1.0));
        }

        #[test]
        fn inverse_x() {
            let p = Point::new(0.0, 1.0, 0.0);
            let half_quarter = Transform::default()
                .rotation_x(std::f64::consts::PI / 4.0)
                .build();
            let inv = half_quarter.inverse();
            assert_eq!(
                &inv * &p,
                Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
            );
        }

        #[test]
        fn y() {
            let half_quarter = Transform::default()
                .rotation_y(std::f64::consts::PI / 4.0)
                .build();

            let full_quarter = Transform::default()
                .rotation_y(std::f64::consts::PI / 2.0)
                .build();

            let p = Point::new(0.0, 0.0, 1.0);

            assert_eq!(
                &half_quarter * &p,
                Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
            );
            assert_eq!(&full_quarter * &p, Point::new(1.0, 0.0, 0.0));
        }

        #[test]
        fn z() {
            let half_quarter = Transform::default()
                .rotation_z(std::f64::consts::PI / 4.0)
                .build();

            let full_quarter = Transform::default()
                .rotation_z(std::f64::consts::PI / 2.0)
                .build();

            let p = Point::new(0.0, 1.0, 0.0);

            assert_eq!(
                &half_quarter * &p,
                Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
            );
            assert_eq!(&full_quarter * &p, Point::new(-1.0, 0.0, 0.0));
        }
    }

    mod shearing {
        use super::*;

        #[test]
        fn x_y() {
            assert_eq!(
                &Transform::default()
                    .shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
                    .build()
                    * &Point::new(2.0, 3.0, 4.0),
                Point::new(5.0, 3.0, 4.0)
            );
        }

        #[test]
        fn x_z() {
            assert_eq!(
                &Transform::default()
                    .shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0)
                    .build()
                    * &Point::new(2.0, 3.0, 4.0),
                Point::new(6.0, 3.0, 4.0)
            );
        }

        #[test]
        fn y_x() {
            assert_eq!(
                &Transform::default()
                    .shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
                    .build()
                    * &Point::new(2.0, 3.0, 4.0),
                Point::new(2.0, 5.0, 4.0)
            );
        }

        #[test]
        fn y_z() {
            assert_eq!(
                &Transform::default()
                    .shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
                    .build()
                    * &Point::new(2.0, 3.0, 4.0),
                Point::new(2.0, 7.0, 4.0)
            );
        }

        #[test]
        fn z_x() {
            assert_eq!(
                &Transform::default()
                    .shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
                    .build()
                    * &Point::new(2.0, 3.0, 4.0),
                Point::new(2.0, 3.0, 6.0)
            );
        }

        #[test]
        fn z_y() {
            assert_eq!(
                &Transform::default()
                    .shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
                    .build()
                    * &Point::new(2.0, 3.0, 4.0),
                Point::new(2.0, 3.0, 7.0)
            );
        }
    }

    mod view {
        use crate::Matrix4;

        use super::*;

        #[test]
        fn default_orientation() {
            let from = Point::new(0.0, 0.0, 0.0);
            let to = Point::new(0.0, 0.0, -1.0);
            let up = Vector::new(0.0, 1.0, 0.0);
            let t = Transform::default().view_transform(from, to, up).build();
            assert_eq!(t, Transform::default().build());
        }

        #[test]
        fn positive_z_orientation() {
            let from = Point::new(0.0, 0.0, 0.0);
            let to = Point::new(0.0, 0.0, 1.0);
            let up = Vector::new(0.0, 1.0, 0.0);
            let t = Transform::default().view_transform(from, to, up).build();
            assert_eq!(t, Transform::default().scaling(-1.0, 1.0, -1.0).build());
        }

        #[test]
        fn move_world() {
            let from = Point::new(0.0, 0.0, 8.0);
            let to = Point::new(0.0, 0.0, 0.0);
            let up = Vector::new(0.0, 1.0, 0.0);
            let t = Transform::default().view_transform(from, to, up).build();
            assert_eq!(t, Transform::default().translation(0.0, 0.0, -8.0).build());
        }

        #[test]
        fn arbitrary() {
            let from = Point::new(1.0, 3.0, 2.0);
            let to = Point::new(4.0, -2.0, 8.0);
            let up = Vector::new(1.0, 1.0, 0.0);
            let t = Transform::default().view_transform(from, to, up).build();
            assert_eq!(
                t,
                Matrix4::new([
                    [-0.50709, 0.50709, 0.67612, -2.36643],
                    [0.76772, 0.60609, 0.12122, -2.82843],
                    [-0.35857, 0.59761, -0.71714, 0.00000],
                    [0.00000, 0.00000, 0.00000, 1.00000]
                ])
            );
        }
    }

    mod ordering {
        use super::*;

        #[test]
        fn sequence() {
            let p = Point::new(1.0, 0.0, 1.0);

            let a = Transform::default()
                .rotation_x(std::f64::consts::PI / 2.0)
                .build();
            let b = Transform::default().scaling(5.0, 5.0, 5.0).build();
            let c = Transform::default().translation(10.0, 5.0, 7.0).build();

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

            let transform = Transform::default()
                .rotation_x(std::f64::consts::PI / 2.0)
                .scaling(5.0, 5.0, 5.0)
                .translation(10.0, 5.0, 7.0)
                .build();

            assert_eq!(&transform * &p, Point::new(15.0, 0.0, 7.0));
        }
    }
}
