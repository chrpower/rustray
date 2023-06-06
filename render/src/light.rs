use core::Colour;
use core::Point;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PointLight {
    position: Point,
    intensity: Colour,
}

impl PointLight {
    pub fn new(position: Point, intensity: Colour) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn intensity(&self) -> &Colour {
        &self.intensity
    }
}

#[cfg(test)]
mod test {
    use crate::PointLight;
    use core::Colour;
    use core::Point;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Colour::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position.clone(), intensity.clone());

        assert_eq!(light.position(), &position);
        assert_eq!(light.intensity(), &intensity);
    }
}
