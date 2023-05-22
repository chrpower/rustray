use geom_lib::{Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
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
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while proj.position.y() > 0.0 {
        proj.update_position();
        proj.update_velocity(&env);
        println!("x: {}, y: {}", proj.position.x(), proj.position.y());
    }
}
