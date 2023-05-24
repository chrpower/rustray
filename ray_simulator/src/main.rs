use draw_lib::{write_ppm, Canvas, PpmWrapper};
use geom_lib::{Colour, Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
    fn tick(&mut self, env: &Environment) {
        self.update_position();
        self.update_velocity(env);
    }

    fn update_position(&mut self) {
        self.position = &self.position + &self.velocity;
    }

    fn update_velocity(&mut self, env: &Environment) {
        self.velocity = &(&self.velocity + &env.gravity) + &env.wind;
    }
}

fn main() {
    let mut proj = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: &Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);
    while proj.position.y() > 0.0 {
        proj.tick(&env);
        canvas
            .write_pixel(
                proj.position.x() as usize,
                (canvas.height() - proj.position.y() as usize) - 1,
                Colour::new(1.0, 0.0, 0.0),
            )
            .unwrap();
    }

    let ppm_wrapper = PpmWrapper::new(canvas, 255);
    write_ppm::<std::fs::File>(&ppm_wrapper, None).unwrap();
}
