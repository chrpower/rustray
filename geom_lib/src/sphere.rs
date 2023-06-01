use crate::{
    util::random_usize, Intersection, Intersections, Material, Point, Ray, SquareMatrix, Vector,
};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub id: usize,
    pub transform: SquareMatrix<4>,
    pub material: Material,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(transform: Option<SquareMatrix<4>>, material: Option<Material>) -> Self {
        Self {
            id: random_usize(),
            transform: transform.unwrap_or(SquareMatrix::identity()),
            material: material.unwrap_or(Material::default()),
        }
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn with_material(&mut self, material: Material) -> &mut Self {
        self.material = material;
        self
    }

    pub fn transform(&self) -> &SquareMatrix<4> {
        &self.transform
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        sphere_intersections(self, &ray.transform(&self.transform.inverse()))
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = &self.transform.inverse() * world_point;
        let object_normal = &object_point - &Point::origin();
        let world_normal = &self.transform.inverse().transpose() * &object_normal;
        world_normal.normalize()
    }
}

pub fn sphere_intersections<'a>(sphere: &'a Sphere, ray: &Ray) -> Intersections<'a> {
    let sphere_to_ray = ray.origin() - &Point::origin();
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * ray.direction().dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return Intersections::new(vec![]);
    }

    Intersections::new(vec![
        Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), sphere),
        Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), sphere),
    ])
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(None, None)
    }
}

use std::fmt::Debug;
pub trait Shape: Debug {
    fn id(&self) -> &usize;
    fn material(&self) -> &Material;
    fn normal_at(&self, world_point: &Point) -> Vector;
}

impl Shape for Sphere {
    fn id(&self) -> &usize {
        self.id()
    }

    fn material(&self) -> &Material {
        self.material()
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        self.normal_at(world_point)
    }
}

#[cfg(test)]
mod test {
    use crate::rotation_z;
    use crate::scaling;
    use crate::sphere::sphere_intersections;
    use crate::translation;
    use crate::Colour;
    use crate::Material;
    use crate::Point;
    use crate::Ray;
    use crate::Sphere;
    use crate::Vector;

    #[test]
    fn changing_a_spheres_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Sphere::new(Some(t.clone()), None);

        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(scaling(2.0, 2.0, 2.0)), None);

        let xs = s.intersect(&r);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 3.0);
        assert_eq!(xs.get(1).unwrap().t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(translation(5.0, 0.0, 0.0)), None);

        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let sphere = Sphere::default();
        let intersections = sphere_intersections(
            &sphere,
            &Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0)),
        );

        assert_eq!(intersections.get(0).unwrap().t(), 4.0);
        assert_eq!(intersections.get(1).unwrap().t(), 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let sphere = Sphere::default();
        let intersections = sphere_intersections(
            &sphere,
            &Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0)),
        );

        assert_eq!(intersections.get(0).unwrap().t(), 5.0);
        assert_eq!(intersections.get(1).unwrap().t(), 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let sphere = Sphere::default();
        let intersections = sphere_intersections(
            &sphere,
            &Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0)),
        );

        assert_eq!(intersections.count(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let sphere = Sphere::default();
        let intersections = sphere_intersections(
            &sphere,
            &Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
        );

        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections.get(0).unwrap().t(), -1.0);
        assert_eq!(intersections.get(1).unwrap().t(), 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let sphere = Sphere::default();

        let intersections = sphere_intersections(
            &sphere,
            &Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0)),
        );

        assert_eq!(intersections.get(0).unwrap().t(), -6.0);
        assert_eq!(intersections.get(1).unwrap().t(), -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        assert_eq!(
            Sphere::default().normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        assert_eq!(
            Sphere::default().normal_at(&Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        assert_eq!(
            Sphere::default().normal_at(&Point::new(0.0, 0.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        assert_eq!(
            Sphere::default().normal_at(&Point::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            )),
            Vector::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let n = Sphere::default().normal_at(&Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new(Some(translation(0.0, 1.0, 0.0)), None);
        let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new(
            Some(&scaling(1.0, 0.5, 1.0) * &rotation_z(std::f64::consts::PI / 5.0)),
            None,
        );
        let n = s.normal_at(&Point::new(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        assert_eq!(Sphere::default().material, Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let c = Colour::new(0.0, 0.0, 0.0);

        let mut s = Sphere::default();
        s.with_material(Material::new(c.clone()));

        assert_eq!(s.material.colour, c);
    }
}
