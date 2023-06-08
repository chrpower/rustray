use crate::World;
use core::Point;
use math::{Matrix4, Ray};
use output::Canvas;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    #[allow(dead_code)]
    field_of_view: f64,
    transform: Matrix4,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, transform: Matrix4) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Self {
            hsize,
            vsize,
            field_of_view,
            transform,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let colour = world.colour_at(ray);
                let _ = image.write_pixel(x, y, colour);
            }
        }

        image
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> math::Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transform_inverse = self.transform.inverse();
        let pixel = &transform_inverse * &Point::new(world_x, world_y, -1.0);
        let origin = &transform_inverse * &Point::new(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).normalize();

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod test {
    use crate::Camera;
    use math::Matrix4;

    mod construction {
        use super::*;

        #[test]
        fn access() {
            let camera = Camera::new(160, 120, std::f64::consts::PI / 2.0, Matrix4::identity());

            assert_eq!(camera.hsize, 160);
            assert_eq!(camera.vsize, 120);
            assert_eq!(camera.field_of_view, std::f64::consts::PI / 2.0);
            assert_eq!(camera.transform, Matrix4::identity());
        }
    }

    mod pixel_size {
        use super::*;

        #[test]
        fn horizontal_canvas() {
            let camera = Camera::new(200, 125, std::f64::consts::PI / 2.0, Matrix4::identity());
            assert!((camera.pixel_size - 0.01).abs() < 1e-8);
        }

        #[test]
        fn vertical_canvas() {
            let camera = Camera::new(125, 200, std::f64::consts::PI / 2.0, Matrix4::identity());
            assert!((camera.pixel_size - 0.01).abs() < 1e-8);
        }
    }

    mod ray_for_pixel {
        use math::Transform;

        use super::*;
        use core::{Point, Vector};

        #[test]
        fn center_of_canvas() {
            let camera = Camera::new(201, 101, std::f64::consts::PI / 2.0, Matrix4::identity());
            let ray = camera.ray_for_pixel(100, 50);

            assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
            assert_eq!(ray.direction, Vector::new(0.0, 0.0, -1.0));
        }

        #[test]
        fn corner_of_canvas() {
            let camera = Camera::new(201, 101, std::f64::consts::PI / 2.0, Matrix4::identity());
            let ray = camera.ray_for_pixel(0, 0);

            assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
            assert_eq!(ray.direction, Vector::new(0.66519, 0.33259, -0.66851));
        }

        #[test]
        fn transformed_camera() {
            let transform = Transform::default()
                .translation(0.0, -2.0, 5.0)
                .rotation_y(std::f64::consts::PI / 4.0)
                .build();
            let camera = Camera::new(201, 101, std::f64::consts::PI / 2.0, transform);
            let ray = camera.ray_for_pixel(100, 50);

            assert_eq!(ray.origin, Point::new(0.0, 2.0, -5.0));
            assert_eq!(
                ray.direction,
                Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
            );
        }
    }
}
