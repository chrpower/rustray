use crate::Shape;

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &dyn Shape {
        self.object
    }
}

#[derive(Debug, Clone)]
pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        let mut intersections = intersections;
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Self { intersections }
    }

    pub fn get(&self, index: usize) -> Option<&Intersection> {
        self.intersections.get(index)
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.t >= 0.0)
    }
}

#[cfg(test)]
mod test {
    use crate::Intersection;
    use crate::Intersections;
    use crate::Point;
    use crate::Ray;
    use crate::Sphere;
    use crate::Vector;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let sphere = Sphere::default();
        let intersection = Intersection::new(3.5, &sphere);

        assert_eq!(intersection.t(), 3.5);
        assert_eq!(intersection.object().id(), sphere.id());
    }

    #[test]
    fn aggregating_intersections() {
        let s1 = Sphere::new(None, None);
        let s2 = Sphere::new(None, None);
        let i1 = Intersection::new(1.0, &s1);
        let i2 = Intersection::new(2.0, &s2);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 1.0);
        assert_eq!(xs.get(1).unwrap().t(), 2.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(None, None);
        let xs = s.intersect(&r);

        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().object().id(), s.id());
        assert_eq!(xs.get(1).unwrap().object().id(), s.id());
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new(None, None);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

        assert_eq!(xs.hit().unwrap().t(), i1.t());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new(None, None);
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

        assert_eq!(xs.hit().unwrap().t(), i2.t());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new(None, None);
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2.clone()]);

        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Sphere::new(None, None);
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);

        assert_eq!(xs.hit().unwrap().t(), i4.t());
    }
}
