use crate::{lighting, PointLight};
use core::{Colour, Point};
use math::Ray;
use shapes::{find_hit, Computations, Intersection, Shape};

pub struct World<'a> {
    pub shapes: Vec<&'a Shape>,
    pub light: PointLight,
}

impl<'a> World<'a> {
    pub fn new(shapes: Vec<&'a Shape>, light: PointLight) -> Self {
        Self { shapes, light }
    }

    pub fn colour_at(&self, ray: Ray) -> Colour {
        let intersections = self.intersect(ray);
        find_hit(&intersections).map_or(Colour::new(0.0, 0.0, 0.0), |hit| {
            self.shade_hit(&hit.prepare_computations(ray))
        })
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection<'a>> {
        self.shapes
            .iter()
            .flat_map(|shape| shape.intersect(&ray))
            .collect()
    }

    pub fn shade_hit(&self, comps: &Computations) -> Colour {
        lighting(
            comps.shape,
            comps.shape.get_material(),
            &self.light,
            &comps.over_point,
            &comps.eye_v,
            &comps.normal_v,
            self.is_shadowed(comps.over_point),
        )
    }

    fn is_shadowed(&self, point: Point) -> bool {
        let v = &self.light.position - &point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect(ray);
        let hit = find_hit(&intersections);

        hit.map_or(false, |hit| hit.t < distance)
    }
}

#[cfg(test)]
mod tests {

    use crate::{PointLight, World};
    use core::{Colour, Point, Vector};
    use math::{Matrix4, Transform};
    use shapes::{Material, Patn, Pattern, Sphere};

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
            let mut material = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
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
            let mut material = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
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
            assert_eq!(colour, Colour::new(0.90495, 0.90495, 0.90495));
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
            let mut material = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
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
            let mut material_outer = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
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

            assert_eq!(world.colour_at(ray), Colour::new(1.0, 1.0, 1.0));
        }
    }

    mod shadow {
        use super::*;

        #[test]
        fn nothing_collinear_with_point_and_light() {
            let mut material = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
            material.diffuse = 0.7;
            material.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material);
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);
            let point = Point::new(0.0, 10.0, 0.0);

            assert!(!world.is_shadowed(point));
        }

        #[test]
        fn object_between_point_and_light() {
            let mut material = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
            material.diffuse = 0.7;
            material.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material);
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);
            let point = Point::new(10.0, -10.0, 10.0);

            assert!(world.is_shadowed(point));
        }

        #[test]
        fn object_behind_light() {
            let mut material = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
            material.diffuse = 0.7;
            material.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material);
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);
            let point = Point::new(-20.0, 20.0, -20.0);

            assert!(!world.is_shadowed(point));
        }

        #[test]
        fn object_behind_point() {
            let mut material = Material::new(Pattern::new(
                Patn::Solid(Colour::new(0.8, 1.0, 0.6)),
                Transform::default().build(),
            ));
            material.diffuse = 0.7;
            material.specular = 0.2;
            let s1 = Sphere::new(Matrix4::identity(), material);
            let s2 = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);
            let point = Point::new(-2.0, 2.0, -2.0);

            assert!(!world.is_shadowed(point));
        }
    }

    mod shade_hit {
        use shapes::Intersection;

        use super::*;

        #[test]
        fn intersection_in_shadow() {
            let s1 = Sphere::new(Matrix4::identity(), Material::default());
            let s2 = Sphere::new(
                Transform::default().translation(0.0, 0.0, 10.0).build(),
                Material::default(),
            );

            let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
            let world = World::new(vec![&s1, &s2], light);

            let ray = math::Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
            let intersection = Intersection::new(4.0, &s2);

            assert_eq!(
                world.shade_hit(&intersection.prepare_computations(ray)),
                Colour::new(0.1, 0.1, 0.1)
            );
        }

        // #[test]
        // fn hit_should_offset_point() {
        //     let s1 = Sphere::new(
        //         Transform::default().translation(0.0, 0.0, 1.0).build(),
        //         Material::default(),
        //     );

        //     let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0));
        //     let world = World::new(vec![&s1], light);

        //     let ray = math::Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        //     let intersection = Intersection::new(5.0, &s1);

        //     assert_eq!(
        //         world.shade_hit(&intersection.prepare_computations(ray)),
        //         Colour::new(0.1, 0.1, 0.1)
        //     );
        // }
    }
}
