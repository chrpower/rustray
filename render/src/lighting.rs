use crate::PointLight;
use core::{Colour, Point, Vector};
use shapes::{Computations, Material};

fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Point,
    eyev: &Vector,
    normalv: &Vector,
) -> Colour {
    let effective_colour = &material.colour * &light.intensity;
    let lightv = (&light.position - point).normalize();
    let ambient = &effective_colour * material.ambient;
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

pub fn shade_hit(light: &PointLight, comps: &Computations) -> Colour {
    lighting(
        comps.shape.material(),
        light,
        &comps.point,
        &comps.eye_v,
        &comps.normal_v,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    mod lighting {
        use super::*;
        use crate::PointLight;
        use core::Colour;
        use core::Point;
        use core::Vector;
        use shapes::Material;

        #[test]
        fn eye_between_the_light_and_the_surface() {
            let result = lighting(
                &Material::default(),
                &PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 0.0, -1.0),
                &Vector::new(0.0, 0.0, -1.0),
            );

            assert_eq!(result, Colour::new(1.9, 1.9, 1.9));
        }

        #[test]
        fn eye_between_light_and_surface_eye_offset_45_degrees() {
            let result = lighting(
                &Material::default(),
                &PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
                &Vector::new(0.0, 0.0, -1.0),
            );

            assert_eq!(result, Colour::new(1.0, 1.0, 1.0));
        }

        #[test]
        fn eye_opposite_surface_light_offset_45_degrees() {
            let result = lighting(
                &Material::default(),
                &PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 0.0, -1.0),
                &Vector::new(0.0, 0.0, -1.0),
            );

            assert_eq!(result, Colour::new(0.7364, 0.7364, 0.7364));
        }

        #[test]
        fn eye_in_the_path_of_the_reflection_vector() {
            let result = lighting(
                &Material::default(),
                &PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
                &Vector::new(0.0, 0.0, -1.0),
            );

            assert_eq!(result, Colour::new(1.6364, 1.6364, 1.6364));
        }

        #[test]
        fn light_behind_the_surface() {
            let result = lighting(
                &Material::default(),
                &PointLight::new(Point::new(0.0, 0.0, 10.0), Colour::new(1.0, 1.0, 1.0)),
                &Point::new(0.0, 0.0, 0.0),
                &Vector::new(0.0, 0.0, -1.0),
                &Vector::new(0.0, 0.0, -1.0),
            );

            assert_eq!(result, Colour::new(0.1, 0.1, 0.1));
        }
    }
}
