use crate::{Colour, Point};

#[derive(Debug, Clone)]
pub struct PointLight {
    pub(crate) position: Point,
    pub(crate) intensity: Colour,
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
    use crate::Colour;
    use crate::Point;
    use crate::PointLight;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Colour::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position.clone(), intensity.clone());

        assert_eq!(light.position(), &position);
        assert_eq!(light.intensity(), &intensity);
    }
}
