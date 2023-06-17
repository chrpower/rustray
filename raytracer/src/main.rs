use output::{write_ppm, PpmWrapper};
use raytracer::room;
use std::f64::consts::PI;

fn main() {
    let canvas = room(4096, 2160, PI / 4.0);
    let ppm_wrapper = PpmWrapper::new(canvas, 255);
    if let Err(e) = write_ppm::<std::fs::File>(&ppm_wrapper) {
        eprintln!("Failed to write PPM file: {}", e);
    }
}
