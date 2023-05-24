use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Colour {
    tuple: Tuple,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Colour {
        Colour {
            tuple: Tuple::new(red, green, blue, 0.0),
        }
    }

    pub fn red(&self) -> f64 {
        self.tuple.x
    }

    pub fn green(&self) -> f64 {
        self.tuple.y
    }

    pub fn blue(&self) -> f64 {
        self.tuple.z
    }
}

use std::cmp::PartialEq;
impl PartialEq for Colour {
    fn eq(&self, other: &Colour) -> bool {
        self.tuple == other.tuple
    }
}

use std::ops::Add;
impl<'a, 'b> Add<&'b Colour> for &'a Colour {
    type Output = Colour;

    fn add(self, other: &'b Colour) -> Colour {
        let result = &self.tuple + &other.tuple;
        Colour::new(result.x, result.y, result.z)
    }
}

use std::ops::Sub;
impl<'a, 'b> Sub<&'b Colour> for &'a Colour {
    type Output = Colour;

    fn sub(self, other: &'b Colour) -> Colour {
        let result = &self.tuple - &other.tuple;
        Colour::new(result.x, result.y, result.z)
    }
}

use std::ops::Mul;
impl<'a> Mul<f64> for &'a Colour {
    type Output = Colour;

    fn mul(self, scalar: f64) -> Colour {
        let result = &self.tuple * scalar;
        Colour::new(result.x, result.y, result.z)
    }
}

impl<'a, 'b> Mul<&'b Colour> for &'a Colour {
    type Output = Colour;

    fn mul(self, other: &'b Colour) -> Colour {
        let result = &self.tuple * &other.tuple;
        Colour::new(result.x, result.y, result.z)
    }
}

#[cfg(test)]
mod test {
    use crate::Colour;

    #[test]
    fn creating_a_colour() {
        let c = Colour::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colours() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        assert_eq!(&c1 + &c2, Colour::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colours() {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        assert_eq!(&c1 - &c2, Colour::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_colour_by_a_scalar() {
        let c = Colour::new(0.2, 0.3, 0.4);
        assert_eq!(&c * 2.0, Colour::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colours() {
        let c1 = Colour::new(1.0, 0.2, 0.4);
        let c2 = Colour::new(0.9, 1.0, 0.1);
        assert_eq!(&c1 * &c2, Colour::new(0.9, 0.2, 0.04));
    }
}
