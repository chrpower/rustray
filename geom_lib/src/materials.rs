use crate::{Colour, Point, PointLight, Vector};

#[derive(Debug, PartialEq)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(colour: Colour) -> Self {
        Self {
            colour,
            ..Default::default()
        }
    }

    pub fn lighting(
        &self,
        light: PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
    ) -> Colour {
        let effective_colour = &self.colour * &light.intensity;
        let lightv = (&light.position - &point).normalize();
        let ambient = &effective_colour * self.ambient;
        let light_dot_normal = lightv.dot(&normalv);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Colour::new(0.0, 0.0, 0.0), Colour::new(0.0, 0.0, 0.0))
        } else {
            let diffuse = &(&effective_colour * self.diffuse) * light_dot_normal;
            let reflectv = -&lightv.reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            let specular = if reflect_dot_eye <= 0.0 {
                Colour::new(0.0, 0.0, 0.0)
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                &(&light.intensity * self.specular) * factor
            };

            (diffuse, specular)
        };

        &(&ambient + &diffuse) + &specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            colour: Colour::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Colour;
    use crate::Material;

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_eq!(m.colour, Colour::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn a_new_colour() {
        let c = Colour::new(0.0, 0.0, 0.0);
        let m = Material::new(c.clone());

        assert_eq!(m.colour, c);
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = crate::Point::new(0.0, 0.0, 0.0);

        let eyev = crate::Vector::new(0.0, 0.0, -1.0);
        let normalv = crate::Vector::new(0.0, 0.0, -1.0);
        let light = crate::PointLight::new(
            crate::Point::new(0.0, 0.0, -10.0),
            crate::Colour::new(1.0, 1.0, 1.0),
        );

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, crate::Colour::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let position = crate::Point::new(0.0, 0.0, 0.0);

        let eyev = crate::Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = crate::Vector::new(0.0, 0.0, -1.0);
        let light = crate::PointLight::new(
            crate::Point::new(0.0, 0.0, -10.0),
            crate::Colour::new(1.0, 1.0, 1.0),
        );

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, crate::Colour::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = crate::Point::new(0.0, 0.0, 0.0);

        let eyev = crate::Vector::new(0.0, 0.0, -1.0);
        let normalv = crate::Vector::new(0.0, 0.0, -1.0);
        let light = crate::PointLight::new(
            crate::Point::new(0.0, 10.0, -10.0),
            crate::Colour::new(1.0, 1.0, 1.0),
        );

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, crate::Colour::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = crate::Point::new(0.0, 0.0, 0.0);

        let eyev = crate::Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = crate::Vector::new(0.0, 0.0, -1.0);
        let light = crate::PointLight::new(
            crate::Point::new(0.0, 10.0, -10.0),
            crate::Colour::new(1.0, 1.0, 1.0),
        );

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, crate::Colour::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = crate::Point::new(0.0, 0.0, 0.0);

        let eyev = crate::Vector::new(0.0, 0.0, -1.0);
        let normalv = crate::Vector::new(0.0, 0.0, -1.0);
        let light = crate::PointLight::new(
            crate::Point::new(0.0, 0.0, 10.0),
            crate::Colour::new(1.0, 1.0, 1.0),
        );

        let result = m.lighting(light, position, eyev, normalv);

        assert_eq!(result, crate::Colour::new(0.1, 0.1, 0.1));
    }
}
