use crate::{Intersection, Material};
use core::{Colour, Point, Vector};
use math::{Matrix4, Ray};

#[derive(Debug)]

pub struct ShapeProperties {
    pub transform: Matrix4,
    pub inverse_transform: Matrix4,
    pub material: Material,
}

#[derive(Debug)]

pub struct Sphere {
    pub properties: ShapeProperties,
}

#[derive(Debug)]

pub struct Plane {
    pub properties: ShapeProperties,
}

#[derive(Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

#[allow(clippy::new_ret_no_self)]
impl Sphere {
    pub fn new(transform: Matrix4, material: Material) -> Shape {
        let inverse_transform = transform.inverse();
        Shape::Sphere(Self {
            properties: ShapeProperties {
                transform,
                inverse_transform,
                material,
            },
        })
    }
}

#[allow(clippy::new_ret_no_self)]
impl Plane {
    pub fn new(transform: Matrix4, material: Material) -> Shape {
        let inverse_transform = transform.inverse();
        Shape::Plane(Self {
            properties: ShapeProperties {
                transform,
                inverse_transform,
                material,
            },
        })
    }
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = ray.transform(self.get_inverse_transform());
        match self {
            Shape::Sphere(_) => {
                let sphere_to_ray = &transformed_ray.origin - &Point::origin();

                let a = transformed_ray.direction.dot(&transformed_ray.direction);
                let b = 2.0 * transformed_ray.direction.dot(&sphere_to_ray);
                let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
                let discriminant = b * b - 4.0 * a * c;

                if discriminant < 0.0 {
                    return vec![];
                }

                vec![
                    Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self),
                    Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self),
                ]
            }
            Shape::Plane(_) => {
                if transformed_ray.direction.y().abs() < 0.0001 {
                    return vec![];
                }

                let t = -transformed_ray.origin.y() / transformed_ray.direction.y();
                vec![Intersection::new(t, self)]
            }
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let world_normal = match self {
            Shape::Sphere(s) => {
                let object_point = &s.properties.inverse_transform * world_point;
                let object_normal = &object_point - &Point::origin();
                &s.properties.inverse_transform.transpose() * &object_normal
            }
            Shape::Plane(p) => {
                let local_normal = Vector::new(0.0, 1.0, 0.0);
                &p.properties.inverse_transform.transpose() * &local_normal
            }
        };
        world_normal.normalize()
    }

    pub fn colour_at(&self, world_point: &Point) -> Colour {
        let properties = match self {
            Shape::Sphere(s) => &s.properties,
            Shape::Plane(p) => &p.properties,
        };
        properties
            .material
            .pattern
            .colour_at_object(self, world_point)
    }

    pub fn get_material(&self) -> &Material {
        let properties = match self {
            Shape::Sphere(s) => &s.properties,
            Shape::Plane(p) => &p.properties,
        };
        &properties.material
    }

    pub fn get_transform(&self) -> &Matrix4 {
        match self {
            Shape::Sphere(s) => &s.properties.transform,
            Shape::Plane(p) => &p.properties.transform,
        }
    }

    pub fn get_inverse_transform(&self) -> &Matrix4 {
        match self {
            Shape::Sphere(s) => &s.properties.inverse_transform,
            Shape::Plane(p) => &p.properties.inverse_transform,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Material;
    use core::{Point, Vector};
    use math::Ray;
    use math::Transform;

    mod plane {
        use super::*;
        use crate::shape::Plane;

        mod intersect {
            use super::*;

            #[test]
            fn ray_parallel() {
                let p = Plane::new(Transform::default().build(), Material::default());
                let xs = p.intersect(&Ray::new(
                    Point::new(0.0, 10.0, 0.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));
                assert_eq!(xs.len(), 0);
            }

            #[test]
            fn ray_coplanar_to_plane() {
                let p = Plane::new(Transform::default().build(), Material::default());
                let xs = p.intersect(&Ray::new(
                    Point::new(0.0, 0.0, 0.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));
                assert_eq!(xs.len(), 0);
            }

            #[test]
            fn ray_from_above() {
                let p = Plane::new(Transform::default().build(), Material::default());
                let xs = p.intersect(&Ray::new(
                    Point::new(0.0, 1.0, 0.0),
                    Vector::new(0.0, -1.0, 0.0),
                ));
                assert_eq!(xs.len(), 1);
                assert_eq!(xs[0].t, 1.0);
            }

            #[test]
            fn ray_from_below() {
                let p = Plane::new(Transform::default().build(), Material::default());
                let xs = p.intersect(&Ray::new(
                    Point::new(0.0, -1.0, 0.0),
                    Vector::new(0.0, 1.0, 0.0),
                ));
                assert_eq!(xs.len(), 1);
                assert_eq!(xs[0].t, 1.0);
            }
        }
    }

    mod sphere {
        use super::*;
        use crate::shape::Sphere;

        mod intersect {
            use super::*;

            #[test]
            fn scaled() {
                let sphere = Sphere::new(
                    Transform::default().scaling(2.0, 2.0, 2.0).build(),
                    Material::default(),
                );

                let xs = sphere.intersect(&Ray::new(
                    Point::new(0.0, 0.0, -5.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));

                assert_eq!(xs.len(), 2);
                assert_eq!(xs[0].t, 3.0);
                assert_eq!(xs[1].t, 7.0);
            }

            #[test]
            fn translated() {
                let sphere = Sphere::new(
                    Transform::default().translation(5.0, 0.0, 0.0).build(),
                    Material::default(),
                );

                let xs = sphere.intersect(&Ray::new(
                    Point::new(0.0, 0.0, -5.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));

                assert!(xs.is_empty());
            }

            #[test]
            fn two_points() {
                let sphere = Sphere::new(Transform::default().build(), Material::default());

                let xs = sphere.intersect(&Ray::new(
                    Point::new(0.0, 0.0, -5.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));

                assert_eq!(xs[0].t, 4.0);
                assert_eq!(xs[1].t, 6.0);
            }

            #[test]
            fn tangent() {
                let sphere = Sphere::new(Transform::default().build(), Material::default());

                let xs = sphere.intersect(&Ray::new(
                    Point::new(0.0, 1.0, -5.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));

                assert_eq!(xs[0].t, 5.0);
                assert_eq!(xs[1].t, 5.0);
            }

            #[test]
            fn misses() {
                let sphere = Sphere::new(Transform::default().build(), Material::default());

                let xs = sphere.intersect(&Ray::new(
                    Point::new(0.0, 2.0, -5.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));

                assert!(xs.is_empty());
            }

            #[test]
            fn originates_inside() {
                let sphere = Sphere::new(Transform::default().build(), Material::default());

                let xs = sphere.intersect(&Ray::new(
                    Point::new(0.0, 0.0, 0.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));

                assert_eq!(xs.len(), 2);
                assert_eq!(xs[0].t, -1.0);
                assert_eq!(xs[1].t, 1.0);
            }

            #[test]
            fn originates_behind() {
                let sphere = Sphere::new(Transform::default().build(), Material::default());

                let xs = sphere.intersect(&Ray::new(
                    Point::new(0.0, 0.0, 5.0),
                    Vector::new(0.0, 0.0, 1.0),
                ));

                assert_eq!(xs[0].t, -6.0);
                assert_eq!(xs[1].t, -4.0);
            }
        }

        mod normal {
            use super::*;

            #[test]
            fn point_on_the_x_axis() {
                assert_eq!(
                    Sphere::new(Transform::default().build(), Material::default())
                        .normal_at(&Point::new(1.0, 0.0, 0.0)),
                    Vector::new(1.0, 0.0, 0.0)
                );
            }

            #[test]
            fn point_on_the_y_axis() {
                assert_eq!(
                    Sphere::new(Transform::default().build(), Material::default())
                        .normal_at(&Point::new(0.0, 1.0, 0.0)),
                    Vector::new(0.0, 1.0, 0.0)
                );
            }

            #[test]
            fn point_on_the_z_axis() {
                assert_eq!(
                    Sphere::new(Transform::default().build(), Material::default())
                        .normal_at(&Point::new(0.0, 0.0, 1.0)),
                    Vector::new(0.0, 0.0, 1.0)
                );
            }

            #[test]
            fn nonaxial_point() {
                assert_eq!(
                    Sphere::new(Transform::default().build(), Material::default()).normal_at(
                        &Point::new(
                            3.0_f64.sqrt() / 3.0,
                            3.0_f64.sqrt() / 3.0,
                            3.0_f64.sqrt() / 3.0
                        )
                    ),
                    Vector::new(
                        3.0_f64.sqrt() / 3.0,
                        3.0_f64.sqrt() / 3.0,
                        3.0_f64.sqrt() / 3.0
                    )
                );
            }

            #[test]
            fn normal_is_a_normalized_vector() {
                let n = Sphere::new(Transform::default().build(), Material::default()).normal_at(
                    &Point::new(
                        3.0_f64.sqrt() / 3.0,
                        3.0_f64.sqrt() / 3.0,
                        3.0_f64.sqrt() / 3.0,
                    ),
                );

                assert_eq!(n, n.normalize());
            }

            #[test]
            fn translated() {
                assert_eq!(
                    Sphere::new(
                        Transform::default().translation(0.0, 1.0, 0.0).build(),
                        Material::default()
                    )
                    .normal_at(&Point::new(0.0, 1.70711, -0.70711)),
                    Vector::new(0.0, 0.70711, -0.70711)
                );
            }

            #[test]
            fn rotation_z() {
                assert_eq!(
                    Sphere::new(
                        Transform::default()
                            .rotation_z(std::f64::consts::PI / 5.0)
                            .scaling(1.0, 0.5, 1.0)
                            .build(),
                        Material::default()
                    )
                    .normal_at(&Point::new(
                        0.0,
                        2.0_f64.sqrt() / 2.0,
                        -2.0_f64.sqrt() / 2.0,
                    )),
                    Vector::new(0.0, 0.97014, -0.24254)
                );
            }
        }
    }
}
