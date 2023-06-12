use core::{Colour, Point, Vector};
use math::Transform;
use output::{write_ppm, PpmWrapper};
use render::PointLight;
use shapes::{Material, Sphere};
use std::f64::consts::PI;

fn main() {
    let mut fm = Material::new(Colour::new(1.0, 0.9, 0.9));
    fm.specular = 0.0;
    let floor = Sphere::new(Transform::default().scaling(10.0, 0.01, 10.0).build(), fm);

    let left_wall = Sphere::new(
        Transform::default()
            .scaling(10.0, 0.01, 10.0)
            .rotation_x(PI / 2.0)
            .rotation_y(-PI / 4.0)
            .translation(0.0, 0.0, 5.0)
            .build(),
        fm,
    );

    let right_wall = Sphere::new(
        Transform::default()
            .scaling(10.0, 0.01, 10.0)
            .rotation_x(PI / 2.0)
            .rotation_y(PI / 4.0)
            .translation(0.0, 0.0, 5.0)
            .build(),
        fm,
    );

    let mut mm = Material::new(Colour::new(0.1, 1.0, 0.5));
    mm.diffuse = 0.7;
    mm.specular = 0.3;
    let middle = Sphere::new(Transform::default().translation(-0.5, 1.0, 0.5).build(), mm);

    let mut rm = Material::new(Colour::new(0.5, 1.0, 0.1));
    rm.diffuse = 0.7;
    rm.specular = 0.3;
    let right = Sphere::new(
        Transform::default()
            .scaling(0.5, 0.5, 0.5)
            .translation(1.5, 0.5, -0.5)
            .build(),
        rm,
    );

    let mut lm = Material::new(Colour::new(1.0, 0.8, 0.1));
    lm.diffuse = 0.7;
    lm.specular = 0.3;
    let left = Sphere::new(
        Transform::default()
            .scaling(0.33, 0.33, 0.33)
            .translation(-1.5, 0.33, -0.75)
            .build(),
        lm,
    );

    let light = PointLight::new(Point::new(0.0, 2.0, 2.0), Colour::new(1.0, 1.0, 1.0));

    let world = render::World::new(
        vec![&floor, &left_wall, &right_wall, &middle, &right, &left],
        light,
    );

    let camera = render::Camera::new(
        8000,
        4000,
        PI / 3.0,
        Transform::default()
            .view_transform(
                Point::new(0.0, 1.5, -5.0),
                Point::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            )
            .build(),
    );

    let canvas = camera.render(&world);

    let ppm_wrapper = PpmWrapper::new(canvas, 255);
    if let Err(e) = write_ppm::<std::fs::File>(&ppm_wrapper) {
        eprintln!("Failed to write PPM file: {}", e);
    }
}
