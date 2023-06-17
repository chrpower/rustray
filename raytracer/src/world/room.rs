use core::{Colour, Point, Vector};
use math::Transform;
use output::Canvas;
use render::{Camera, PointLight, World};
use shapes::{Material, Plane, Sphere};
use std::f64::consts::PI;

pub fn room(h_res: usize, v_res: usize, focal_length: f64) -> Canvas {
    let mut material1 = Material::new(Colour::new(1.0, 0.0, 0.0)); // Red
    material1.diffuse = 0.7;
    material1.specular = 0.3;
    let sphere1 = Sphere::new(
        Transform::default()
            .scaling(1.0, 1.0, 1.0) // Full size
            .translation(-2.0, -1.5, 2.0) // Positioned further left and forward
            .build(),
        material1,
    );

    let mut material2 = Material::new(Colour::new(0.0, 1.0, 0.0)); // Green
    material2.diffuse = 0.7;
    material2.specular = 0.3;
    let sphere2 = Sphere::new(
        Transform::default()
            .scaling(1.25, 1.25, 1.25) // Double size
            .translation(1.5, -0.5, -2.5) // Positioned further right and backward
            .build(),
        material2,
    );

    let mut material3 = Material::new(Colour::new(1.0, 0.5, 0.0)); // Orange
    material3.diffuse = 0.7;
    material3.specular = 0.3;
    let sphere3 = Sphere::new(
        Transform::default()
            .scaling(1.125, 1.125, 1.125) // 1.5 times the original size
            .translation(0.0, 0.25, -1.0) // Positioned higher in the top center
            .build(),
        material3,
    );

    let mut floor_material = Material::new(Colour::new(0.6, 0.8, 1.0)); // Light Blue
    floor_material.diffuse = 0.85; // Matte finish
    floor_material.specular = 0.15; // Not very reflective
    let floor = Plane::new(
        Transform::default().translation(0.0, -3.0, 0.0).build(),
        floor_material,
    );

    let mut ceiling_material = Material::new(Colour::new(0.8, 0.9, 1.0)); // Lighter Blue
    ceiling_material.diffuse = 0.85; // Matte finish
    ceiling_material.specular = 0.15; // Not very reflective
    let ceiling = Plane::new(
        Transform::default()
            .rotation_x(PI)
            .translation(0.0, 2.0, 0.0)
            .build(),
        ceiling_material,
    );

    let mut back_wall_material = Material::new(Colour::new(0.7, 0.85, 1.0)); // Mid Light Blue
    back_wall_material.diffuse = 0.85; // Matte finish
    back_wall_material.specular = 0.15; // Not very reflective

    let back_wall = Plane::new(
        Transform::default()
            .rotation_x(PI / 2.0)
            .translation(0.0, 0.0, 3.0)
            .build(),
        back_wall_material,
    );

    let mut right_wall_material = Material::new(Colour::new(0.75, 0.88, 1.0)); // Brighter Light Blue
    right_wall_material.diffuse = 0.85; // Matte finish
    right_wall_material.specular = 0.15; // Not very reflective
    let right_wall = Plane::new(
        Transform::default()
            .rotation_x(PI / 2.0)
            .translation(0.0, 0.0, 4.0)
            .rotation_y(PI / 2.0)
            .build(),
        right_wall_material,
    );

    let mut left_wall_material = Material::new(Colour::new(0.65, 0.82, 1.0)); // Darker Light Blue
    left_wall_material.diffuse = 0.85; // Matte finish
    left_wall_material.specular = 0.15; // Not very reflective

    let left_wall = Plane::new(
        Transform::default()
            .rotation_x(PI / 2.0)
            .translation(0.0, 0.0, 4.0)
            .rotation_y(-PI / 2.0)
            .build(),
        left_wall_material,
    );

    let light = PointLight::new(Point::new(-2.0, -1.5, -2.0), Colour::new(1.0, 1.0, 1.0));

    let world = World::new(
        vec![
            &sphere1,
            &sphere2,
            &sphere3,
            &floor,
            &back_wall,
            &ceiling,
            &right_wall,
            &left_wall,
        ],
        light,
    );

    let camera = Camera::new(
        h_res,
        v_res,
        focal_length,
        Transform::default()
            .view_transform(
                Point::new(0.0, 0.0, -12.0),
                Point::new(0.0, -0.40, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            )
            .build(),
    );

    camera.render(&world)
}
