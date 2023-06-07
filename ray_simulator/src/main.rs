use core::{Colour, Point};
use math::{Matrix4, Ray};
use output::{Canvas, PpmWrapper, write_ppm};
use render::PointLight;
use shapes::{find_hit, Material, Sphere};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_pixels = 1000;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let m = Matrix4::identity();
    let mat = Material::new(Colour::new(1.0, 0.2, 1.0));
    let sphere = Sphere::new(&m, &mat);

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (&position - &ray_origin).normalize());

            let intersections = sphere.intersect(&ray);
            if let Some(intersection) =
                find_hit([[intersections[0].as_ref(), intersections[1].as_ref()]])
            {
                let intersected_shape = intersection.get_shape();

                let point = ray.position(intersection.t());
                let normal = intersected_shape.normal_at(&point);
                let eye = -ray.direction();

                let colour = intersected_shape
                    .material()
                    .lighting(&light, &point, &eye, &normal);
                canvas.write_pixel(x, y, colour).unwrap();
            }
        }
    }

    let ppm_wrapper = PpmWrapper::new(canvas, 255);
    if let Err(e) = write_ppm::<std::fs::File>(&ppm_wrapper) {
        eprintln!("Failed to write PPM file: {}", e);
    }
}
