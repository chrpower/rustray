use core::{Colour, Point};
use math::Matrix4;

use crate::Shape;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Patn {
    Solid(Colour),
    Stripe(Colour, Colour),
    Stripes(Colour, Colour, Colour),
    Gradient(Colour, Colour),
    Ring(Colour, Colour),
    Rings(Colour, Colour, Colour),
    Checkers(Colour, Colour),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pattern {
    pub pattern: Patn,
    pub transform: Matrix4,
    pub transform_inverse: Matrix4,
}

impl Pattern {
    pub fn new(pattern: Patn, transform: Matrix4) -> Self {
        Self {
            pattern,
            transform,
            transform_inverse: transform.inverse(),
        }
    }

    pub fn colour_at(&self, point: &Point) -> Colour {
        match self.pattern {
            Patn::Solid(c) => c,
            Patn::Stripe(c1, c2) => {
                if point.x().floor() % 2.0 == 0.0 {
                    c1
                } else {
                    c2
                }
            }
            Patn::Stripes(c1, c2, c3) => {
                let x = point.x();
                let fraction = x - x.floor();

                if fraction < 1.0 / 3.0 {
                    c1
                } else if fraction < 2.0 / 3.0 {
                    c2
                } else {
                    c3
                }
            }
            Patn::Gradient(c1, c2) => {
                let distance = &c2 - &c1;
                let fraction = point.x() - point.x().floor();
                &c1 + &(&distance * fraction)
            }

            Patn::Ring(c1, c2) => {
                let dist = (point.x().powi(2) + point.z().powi(2)).sqrt().floor();
                if dist % 2.0 == 0.0 {
                    c1
                } else {
                    c2
                }
            }
            Patn::Rings(c1, c2, c3) => {
                let dist = (point.x().powi(2) + point.z().powi(2)).sqrt().floor();
                if dist % 2.0 == 0.0 {
                    c1
                } else if dist % 3.0 == 0.0 {
                    c2
                } else {
                    c3
                }
            }
            Patn::Checkers(c1, c2) => {
                if (point.x().floor() + point.y().floor() + point.z().floor()) % 2.0 == 0.0 {
                    c1
                } else {
                    c2
                }
            }
        }
    }

    pub fn colour_at_object(&self, shape: &dyn Shape, world_point: &Point) -> Colour {
        let object_point = shape.get_inverse_transform() * world_point;
        let pattern_point = self.world_to_pattern(&object_point);
        self.colour_at(&pattern_point)
    }

    fn world_to_pattern(&self, object_point: &Point) -> Point {
        match self.pattern {
            Patn::Solid(_) => *object_point,
            _ => &self.transform_inverse * object_point,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Material, Patn, Pattern, Sphere};
    use core::{Colour, Point};
    use math::Transform;

    mod stripped {
        use super::*;

        #[test]
        fn stripes_with_an_object_transformation() {
            let p = Pattern::new(
                Patn::Stripe(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().build(),
            );

            let sphere = Sphere::new(
                Transform::default().scaling(2.0, 2.0, 2.0).build(),
                Material::new(p),
            );

            let c = p.colour_at_object(&sphere, &Point::new(1.5, 0.0, 0.0));

            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));
        }

        #[test]
        fn stripes_with_a_pattern_transformation() {
            let p = Pattern::new(
                Patn::Stripe(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().scaling(2.0, 2.0, 2.0).build(),
            );

            let sphere = Sphere::new(Transform::default().build(), Material::new(p));

            let c = p.colour_at_object(&sphere, &Point::new(1.5, 0.0, 0.0));

            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));
        }

        #[test]
        fn stripes_with_both_an_object_and_a_pattern_transformation() {
            let p = Pattern::new(
                Patn::Stripe(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().scaling(2.0, 2.0, 2.0).build(),
            );

            let sphere = Sphere::new(
                Transform::default().scaling(0.5, 0.5, 0.5).build(),
                Material::new(p),
            );

            let c = p.colour_at_object(&sphere, &Point::new(2.5, 0.0, 0.0));

            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));
        }
    }

    mod gradient {
        use super::*;

        #[test]
        fn a_gradient() {
            let p = Pattern::new(
                Patn::Gradient(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().build(),
            );

            let c = p.colour_at(&Point::new(0.0, 0.0, 0.0));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(0.25, 0.0, 0.0));
            assert_eq!(c, Colour::new(0.75, 0.75, 0.75));

            let c = p.colour_at(&Point::new(0.5, 0.0, 0.0));
            assert_eq!(c, Colour::new(0.5, 0.5, 0.5));

            let c = p.colour_at(&Point::new(0.75, 0.0, 0.0));
            assert_eq!(c, Colour::new(0.25, 0.25, 0.25));
        }
    }

    mod ring {
        use super::*;

        #[test]
        fn extend_in_both_x_and_z() {
            let p = Pattern::new(
                Patn::Ring(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().build(),
            );

            let c = p.colour_at(&Point::new(0.0, 0.0, 0.0));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(1.0, 0.0, 0.0));
            assert_eq!(c, Colour::new(0.0, 0.0, 0.0));

            let c = p.colour_at(&Point::new(0.0, 0.0, 1.0));
            assert_eq!(c, Colour::new(0.0, 0.0, 0.0));

            let c = p.colour_at(&Point::new(0.708, 0.0, 0.708));
            assert_eq!(c, Colour::new(0.0, 0.0, 0.0));
        }
    }

    mod checkers {
        use super::*;

        #[test]
        fn repeat_in_x() {
            let p = Pattern::new(
                Patn::Checkers(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().build(),
            );

            let c = p.colour_at(&Point::new(0.0, 0.0, 0.0));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(0.99, 0.0, 0.0));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(1.01, 0.0, 0.0));
            assert_eq!(c, Colour::new(0.0, 0.0, 0.0));
        }

        #[test]
        fn repeat_in_y() {
            let p = Pattern::new(
                Patn::Checkers(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().build(),
            );

            let c = p.colour_at(&Point::new(0.0, 0.0, 0.0));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(0.0, 0.99, 0.0));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(0.0, 1.01, 0.0));
            assert_eq!(c, Colour::new(0.0, 0.0, 0.0));
        }

        #[test]
        fn repeat_in_z() {
            let p = Pattern::new(
                Patn::Checkers(Colour::new(1.0, 1.0, 1.0), Colour::new(0.0, 0.0, 0.0)),
                Transform::default().build(),
            );

            let c = p.colour_at(&Point::new(0.0, 0.0, 0.0));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(0.0, 0.0, 0.99));
            assert_eq!(c, Colour::new(1.0, 1.0, 1.0));

            let c = p.colour_at(&Point::new(0.0, 0.0, 1.01));
            assert_eq!(c, Colour::new(0.0, 0.0, 0.0));
        }
    }
}
