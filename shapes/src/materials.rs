use core::Colour;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    use crate::Material;
    use core::Colour;

    mod creation {
        use super::*;

        #[test]
        fn access() {
            let m = Material::new(Colour::new(1.0, 1.0, 1.0));

            assert_eq!(m.colour, Colour::new(1.0, 1.0, 1.0));
            assert_eq!(m.ambient, 0.1);
            assert_eq!(m.diffuse, 0.9);
            assert_eq!(m.specular, 0.9);
            assert_eq!(m.shininess, 200.0);
        }

        #[test]
        fn default() {
            let m = Material::default();

            assert_eq!(m.colour, Colour::new(1.0, 1.0, 1.0));
            assert_eq!(m.ambient, 0.1);
            assert_eq!(m.diffuse, 0.9);
            assert_eq!(m.specular, 0.9);
            assert_eq!(m.shininess, 200.0);
        }
    }
}
