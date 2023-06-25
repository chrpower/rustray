use core::{Colour, Point, Vector};
use math::Transform;
use output::Canvas;
use render::{Camera, PointLight, World};
use shapes::{Material, Patn, Pattern, Plane, Sphere};

pub fn pattern(h_res: usize, v_res: usize, focal_length: f64) -> Canvas {
    let mut fm = Material::new(Pattern::new(
        Patn::Checkers(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.5, 0.5)),
        Transform::default().build(),
    ));
    fm.diffuse = 0.85;
    fm.specular = 0.15;
    let floor = Plane::new(Transform::default().build(), fm);

    let mut wm = Material::new(Pattern::new(
        Patn::Rings(
            Colour::new(0.5, 0.5, 0.5),
            Colour::new(1.0, 1.0, 1.0),
            Colour::new(0.7, 0.6, 0.7),
        ),
        Transform::default()
            .shearing(1.0, 1.0, 0.0, 0.0, 0.0, 0.0)
            .build(),
    ));
    wm.diffuse = 0.85;
    wm.specular = 0.15;

    let wall = Plane::new(
        Transform::default()
            .rotation_x(1.571)
            .translation(0.0, 0.0, 5.0)
            .build(),
        wm,
    );

    let mut m1 = Material::new(Pattern::new(
        Patn::Ring(Colour::new(1.0, 1.0, 1.0), Colour::new(0.7, 0.6, 0.7)),
        Transform::default().scaling(0.2, 0.2, 0.2).build(),
    ));
    m1.diffuse = 0.7;
    m1.specular = 0.3;
    let s1 = Sphere::new(
        Transform::default()
            .scaling(1.5, 1.5, 1.5)
            .rotation_x(1.5)
            .translation(-3.0, 1.5, -4.0)
            .build(),
        m1,
    );

    let mut m2 = Material::new(Pattern::new(
        Patn::Stripes(
            Colour::new(0.5, 0.5, 0.5),
            Colour::new(1.0, 1.0, 1.0),
            Colour::new(0.7, 0.6, 0.7),
        ),
        Transform::default().build(),
    ));
    m2.diffuse = 0.7;
    m2.specular = 0.3;
    let s2 = Sphere::new(
        Transform::default()
            .scaling(1.5, 1.5, 1.5)
            .translation(3.0, 1.5, -4.0)
            .build(),
        m2,
    );

    let mut m3 = Material::new(Pattern::new(
        Patn::Gradient(Colour::new(0.7, 0.6, 0.7), Colour::new(0.0, 0.0, 0.0)),
        Transform::default()
            .scaling(2.0, 2.0, 2.0)
            .translation(1.0, 0.0, 0.0)
            .build(),
    ));
    m3.diffuse = 0.7;
    m3.specular = 0.3;
    let s3 = Sphere::new(
        Transform::default()
            .scaling(0.33, 0.33, 0.33)
            .translation(0.0, 1.0, -7.0)
            .build(),
        m3,
    );

    let mut m4 = Material::new(Pattern::new(
        Patn::Rings(
            Colour::new(0.5, 0.5, 0.5),
            Colour::new(1.0, 1.0, 1.0),
            Colour::new(0.7, 0.6, 0.7),
        ),
        Transform::default()
            .scaling(0.2, 0.2, 0.2)
            .shearing(0.0, 0.0, 0.0, 0.0, 1.0, 1.0)
            .build(),
    ));
    m4.diffuse = 0.7;
    m4.specular = 0.3;
    let s4 = Sphere::new(
        Transform::default()
            .scaling(0.66, 0.11, 0.66)
            .translation(-2.0, 0.05, -6.25)
            .build(),
        m4,
    );

    let light = PointLight::new(Point::new(-7.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
    let world = World::new(vec![&floor, &wall, &s1, &s2, &s3, &s4], light);

    let camera = Camera::new(
        h_res,
        v_res,
        focal_length,
        Transform::default()
            .view_transform(
                Point::new(-1.0, 2.0, -9.0),
                Point::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            )
            .build(),
    );

    camera.render(&world)
}
