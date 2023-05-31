use draw_lib::{write_ppm, Canvas, PpmWrapper};
use geom_lib::{scaling, translation, Colour, Point, SquareMatrix, Vector};

fn main() {
    plot_clock();
    plot_projectile();
}

fn plot_clock() {
    const CANVAS_SIZE: usize = 250;
    const CANVAS_CENTER: f64 = CANVAS_SIZE as f64 / 2.0;
    const CLOCK_RADIUS: f64 = CANVAS_SIZE as f64 * (3.0 / 8.0);

    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    let position_transformation =
        &translation(CANVAS_CENTER, CANVAS_CENTER, 0.0) * &scaling(CLOCK_RADIUS, CLOCK_RADIUS, 0.0);

    const HOUR_ROTATION: f64 = std::f64::consts::PI / 6.0;
    let clock_rotation = geom_lib::rotation_z(HOUR_ROTATION);

    let mut hand_position = Point::new(0.0, 1.0, 0.0);
    draw_clock_hand(&mut canvas, &position_transformation, &hand_position);

    for _ in 0..12 {
        hand_position = &clock_rotation * &hand_position;
        draw_clock_hand(&mut canvas, &position_transformation, &hand_position);
    }

    let ppm_wrapper = PpmWrapper::new(canvas, 255);
    if let Err(e) = write_ppm::<std::fs::File>(&ppm_wrapper, None) {
        eprintln!("Failed to write PPM file: {}", e);
    }
}

fn draw_clock_hand(
    canvas: &mut Canvas,
    position_transformation: &SquareMatrix<4>,
    hand_position: &Point,
) {
    let clock_point = position_transformation * hand_position;
    canvas
        .write_pixel(
            clock_point.x() as usize,
            clock_point.y() as usize,
            Colour::new(1.0, 0.0, 0.0),
        )
        .unwrap();
}

fn plot_projectile() {
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
