use geom_lib::Colour;

#[allow(dead_code)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Colour>,
}

impl Canvas {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Colour::new(0.0, 0.0, 0.0); width * height],
        }
    }

    #[allow(dead_code)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[allow(dead_code)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[allow(dead_code)]
    pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) -> Result<(), String> {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = colour;
            Ok(())
        } else {
            let error_message = format!(
                "Pixel indices ({}, {}) are out of bounds for a canvas of size {}x{}.",
                x, y, self.width, self.height
            );
            Err(error_message)
        }
    }

    #[allow(dead_code)]
    pub fn pixel_at(&self, x: usize, y: usize) -> Result<&Colour, String> {
        if x < self.width && y < self.height {
            Ok(&self.pixels[y * self.width + x])
        } else {
            let error_message = format!(
                "Pixel indices ({}, {}) are out of bounds for a canvas of size {}x{}.",
                x, y, self.width, self.height
            );
            Err(error_message)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::canvas::Colour;

    #[test]
    fn write_and_read_valid_pixel() {
        let mut canvas = Canvas::new(10, 10);
        let red = Colour::new(1.0, 0.0, 0.0);
        let result = canvas.write_pixel(5, 5, red.clone());
        assert!(result.is_ok());
        assert_eq!(*canvas.pixel_at(5, 5).unwrap(), red);
    }

    #[test]
    fn write_invalid_pixel_out_of_bounds() {
        let mut canvas = Canvas::new(10, 10);
        let red = Colour::new(1.0, 0.0, 0.0);
        let result = canvas.write_pixel(15, 15, red);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Pixel indices (15, 15) are out of bounds for a canvas of size 10x10."
        );
    }

    #[test]
    fn read_invalid_pixel_out_of_bounds() {
        let canvas = Canvas::new(10, 10);
        let result = canvas.pixel_at(15, 15);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Pixel indices (15, 15) are out of bounds for a canvas of size 10x10."
        );
    }

    #[test]
    fn write_and_read_multiple_pixels() {
        let mut canvas = Canvas::new(10, 10);
        let red = Colour::new(1.0, 0.0, 0.0);
        let green = Colour::new(0.0, 1.0, 0.0);
        let blue = Colour::new(0.0, 0.0, 1.0);

        assert!(canvas.write_pixel(2, 2, red.clone()).is_ok());
        assert!(canvas.write_pixel(5, 5, green.clone()).is_ok());
        assert!(canvas.write_pixel(8, 8, blue.clone()).is_ok());

        assert_eq!(*canvas.pixel_at(2, 2).unwrap(), red);
        assert_eq!(*canvas.pixel_at(5, 5).unwrap(), green);
        assert_eq!(*canvas.pixel_at(8, 8).unwrap(), blue);
    }
}
