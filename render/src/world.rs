use crate::{shade_hit, PointLight};
use core::Colour;
use math::Ray;
use shapes::{find_hit, Intersection, Shape};

pub struct World<'a> {
    pub shapes: Vec<&'a dyn Shape>,
    pub light: PointLight,
}

impl<'a> World<'a> {
    pub fn new(shapes: Vec<&'a dyn Shape>, light: PointLight) -> Self {
        Self { shapes, light }
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        let intersections = self.intersect(ray);
        find_hit(&intersections).map_or(Colour::new(0.0, 0.0, 0.0), |hit| {
            shade_hit(&self.light, &hit.prepare_computations(ray))
        })
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection<'a>> {
        self.shapes
            .iter()
            .flat_map(|shape| shape.intersect(&ray))
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::{PointLight, World};
    use core::{Colour, Point, Vector};
    use math::{Matrix4, Transform};
    use shapes::{Material, Sphere};

    mod contruction {
        use super::*;

        #[test]
        fn empty() {
            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![], light);

            assert_eq!(world.light, light);
            assert_eq!(world.shapes.len(), 0);
        }

        #[test]
        fn two_spheres() {
            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let s1 = Sphere::new(Matrix4::identity(), Material::default());
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let world = World::new(vec![&s1, &s2], light);
            assert_eq!(world.light, light);
            assert_eq!(world.shapes.len(), 2);
        }
    }

    mod intersect {
        use math::Ray;
        use shapes::find_hit;

        use super::*;

        #[test]
        fn world_with_ray() {
            let s1 = Sphere::new(Matrix4::identity(), Material::default());
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);

            let ray = math::Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
            let intersections = world.intersect(Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));

            assert_eq!(intersections.len(), 4);
            assert_eq!(find_hit(&intersections).unwrap().t, 4.0);
        }
    }

    mod shading {
        use super::*;
        use math::Ray;

        #[test]
        fn shade_intersection() {
            let mut material = Material::new(Colour::new(0.8, 1.0, 0.6));
            material.diffuse = 0.7;
            material.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material);
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);

            let colour = world.colour_at(Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
            ));
            assert_eq!(colour, Colour::new(0.38066, 0.47583, 0.2855));
        }

        #[test]
        fn shade_intersection_inside() {
            let mut material = Material::new(Colour::new(0.8, 1.0, 0.6));
            material.diffuse = 0.7;
            material.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material);
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(0.0, 0.25, 0.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);

            let colour = world.colour_at(Ray::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
            ));
            assert_eq!(colour, Colour::new(0.90498, 0.90498, 0.90498));
        }

        #[test]
        fn colour_when_ray_misses() {
            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let s1 = Sphere::new(Matrix4::identity(), Material::default());
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );
            let world = World::new(vec![&s1, &s2], light);

            let ray = math::Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

            assert_eq!(world.colour_at(ray), Colour::new(0.0, 0.0, 0.0));
        }

        #[test]
        fn colour_when_a_ray_hits() {
            let mut material = Material::new(Colour::new(0.8, 1.0, 0.6));
            material.diffuse = 0.7;
            material.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material);
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);

            let ray = math::Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

            assert_eq!(world.colour_at(ray), Colour::new(0.38066, 0.47583, 0.2855));
        }

        #[test]
        fn colour_with_intersection_behind_ray() {
            let mut material_outer = Material::new(Colour::new(0.8, 1.0, 0.6));
            material_outer.ambient = 1.0;
            material_outer.diffuse = 0.7;
            material_outer.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material_outer);

            let mut material_inner = Material::default();
            material_inner.ambient = 1.0;

            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                material_inner,
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);

            let ray = math::Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

            assert_eq!(world.colour_at(ray), material_inner.colour);
        }
    }
}
