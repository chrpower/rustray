use core::{Colour, Point, Vector};
use math::Transform;
use output::Canvas;
use render::{Camera, PointLight, World};
use shapes::{Material, Plane, Sphere};

pub fn simple_plane(h_res: usize, v_res: usize, focal_length: f64) -> Canvas {
    let mut fm = Material::new(Colour::new(1.0, 1.0, 1.0));
    fm.diffuse = 0.85;
    fm.specular = 0.15;

    let floor = Plane::new(Transform::default().build(), fm);

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

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
    let world = World::new(vec![&floor, &middle, &left, &right], light);

    let camera = Camera::new(
        h_res,
        v_res,
        focal_length,
        Transform::default()
            .view_transform(
                Point::new(0.0, 1.5, -5.0),
                Point::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            )
            .build(),
    );

    camera.render(&world)
}
