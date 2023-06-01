use crate::{util::random_usize, Intersection, Intersections, Point, Ray, SquareMatrix};

#[allow(dead_code)]
pub struct Sphere {
    pub id: usize,
    pub transform: SquareMatrix<4>,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(transform: Option<SquareMatrix<4>>) -> Self {
        Self {
            id: random_usize(),
            transform: transform.unwrap_or(SquareMatrix::identity()),
        }
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn transform(&self) -> &SquareMatrix<4> {
        &self.transform
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        sphere_intersections(self, &ray.transform(&self.transform.inverse()))
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
        Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), sphere.id()),
        Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), sphere.id()),
    ])
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod test {
    use crate::scaling;
    use crate::sphere::sphere_intersections;
    use crate::translation;
    use crate::Point;
    use crate::Ray;
    use crate::Sphere;
    use crate::Vector;

    #[test]
    fn changing_a_spheres_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Sphere::new(Some(t.clone()));

        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(scaling(2.0, 2.0, 2.0)));

        let xs = s.intersect(&r);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 3.0);
        assert_eq!(xs.get(1).unwrap().t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Some(translation(5.0, 0.0, 0.0)));

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
}
