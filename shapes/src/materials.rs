use crate::{pattern::Patn, Pattern};
use core::Colour;
use math::Matrix4;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(pattern: Pattern) -> Self {
        Self {
            pattern,
            ..Default::default()
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Pattern::new(Patn::Solid(Colour::new(1.0, 1.0, 1.0)), Matrix4::identity()),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Material, Pattern};
    use core::Colour;

    mod creation {
        use math::Matrix4;

        use crate::pattern::Patn;

        use super::*;

        #[test]
        fn access() {
            let m = Material::new(Pattern::new(
                Patn::Solid(Colour::new(1.0, 1.0, 1.0)),
                Matrix4::identity(),
            ));

            assert_eq!(
                m.pattern,
                Pattern::new(Patn::Solid(Colour::new(1.0, 1.0, 1.0)), Matrix4::identity())
            );
            assert_eq!(m.ambient, 0.1);
            assert_eq!(m.diffuse, 0.9);
            assert_eq!(m.specular, 0.9);
            assert_eq!(m.shininess, 200.0);
        }

        #[test]
        fn default() {
            let m = Material::default();

            assert_eq!(
                m.pattern,
                Pattern::new(Patn::Solid(Colour::new(1.0, 1.0, 1.0)), Matrix4::identity())
            );
            assert_eq!(m.ambient, 0.1);
            assert_eq!(m.diffuse, 0.9);
            assert_eq!(m.specular, 0.9);
            assert_eq!(m.shininess, 200.0);
        }
    }
}
