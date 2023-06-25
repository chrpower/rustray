use crate::PointLight;
use core::{Colour, Point, Vector};
use shapes::{Material, Shape};

pub fn lighting(
    shape: &dyn Shape,
    material: &Material,
    light: &PointLight,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
    in_shadow: bool,
) -> Colour {
    let effective_colour = &shape.colour_at(point) * &light.intensity;
    let ambient = &effective_colour * material.ambient;

    if in_shadow {
        return ambient;
    }

    let lightv = (&light.position - point).normalize();
    let light_dot_normal = lightv.dot(normalv);

    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (Colour::new(0.0, 0.0, 0.0), Colour::new(0.0, 0.0, 0.0))
    } else {
        let diffuse = &(&effective_colour * material.diffuse) * light_dot_normal;
        let reflectv = -&lightv.reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        let specular = if reflect_dot_eye <= 0.0 {
            Colour::new(0.0, 0.0, 0.0)
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            &(&light.intensity * material.specular) * factor
        };

        (diffuse, specular)
    };

    &(&ambient + &diffuse) + &specular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PointLight;
    use core::Colour;
    use core::Point;
    use core::Vector;
    use math::Matrix4;
    use shapes::Material;
    use shapes::Sphere;
    mod lighting {
        use super::*;

        #[test]
        fn eye_between_the_light_and_the_surface() {
            let result = lighting(
                &Sphere::new(Matrix4::identity(), Material::default()),
                &Material::default(),
                &PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 0.0, -1.0),
                &Vector::new(0.0, 0.0, -1.0),
                false,
            );

            assert_eq!(result, Colour::new(1.9, 1.9, 1.9));
        }

        #[test]
        fn eye_between_light_and_surface_eye_offset_45_degrees() {
            let result = lighting(
                &Sphere::new(Matrix4::identity(), Material::default()),
                &Material::default(),
                &PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
                &Vector::new(0.0, 0.0, -1.0),
                false,
            );

            assert_eq!(result, Colour::new(1.0, 1.0, 1.0));
        }

        #[test]
        fn eye_opposite_surface_light_offset_45_degrees() {
            let result = lighting(
                &Sphere::new(Matrix4::identity(), Material::default()),
                &Material::default(),
                &PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 0.0, -1.0),
                &Vector::new(0.0, 0.0, -1.0),
                false,
            );

            assert_eq!(result, Colour::new(0.7364, 0.7364, 0.7364));
        }

        #[test]
        fn eye_in_the_path_of_the_reflection_vector() {
            let result = lighting(
                &Sphere::new(Matrix4::identity(), Material::default()),
                &Material::default(),
                &PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
                &Vector::new(0.0, 0.0, -1.0),
                false,
            );

            assert_eq!(result, Colour::new(1.6364, 1.6364, 1.6364));
        }

        #[test]
        fn light_behind_the_surface() {
            let result = lighting(
                &Sphere::new(Matrix4::identity(), Material::default()),
                &Material::default(),
                &PointLight::new(Point::new(0.0, 0.0, 10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 0.0, -1.0),
                &Vector::new(0.0, 0.0, -1.0),
                false,
            );

            assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
        }
    }

    mod shadow {
        use super::*;
        #[test]
        fn lighting_with_the_surface_in_shadow() {
            let result = lighting(
                &Sphere::new(Matrix4::identity(), Material::default()),
                &Material::default(),
                &PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 0.0, -1.0),
                &Vector::new(0.0, 0.0, -1.0),
                true,
            );

            assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
        }
    }
}
