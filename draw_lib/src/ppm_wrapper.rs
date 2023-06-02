use crate::canvas::Canvas;

pub struct PpmWrapper {
    canvas: Canvas,
    max_colour_value: usize,
}
impl PpmWrapper {
    pub fn new(canvas: Canvas, max_colour_value: usize) -> PpmWrapper {
        PpmWrapper {
            canvas,
            max_colour_value,
        }
    }

    pub fn height(&self) -> usize {
        self.canvas.height()
    }

    pub fn width(&self) -> usize {
        self.canvas.width()
    }

    fn generate_header(&self) -> String {
        format!(
            "P3\n{} {}\n{}\n",
            self.width(),
            self.height(),
            self.max_colour_value
        )
    }

    fn generate_body(&self) -> String {
        let mut body = String::new();
        for y in 0..self.canvas.height() {
            let mut line = String::new();
            for x in 0..self.canvas.width() {
                let pixel = self.canvas.pixel_at(x, y).unwrap();
                let red = scale_colour_value(pixel.red(), self.max_colour_value);
                let green = scale_colour_value(pixel.green(), self.max_colour_value);
                let blue = scale_colour_value(pixel.blue(), self.max_colour_value);

                add_to_line_or_body(&mut body, &mut line, red);
                add_to_line_or_body(&mut body, &mut line, green);
                add_to_line_or_body(&mut body, &mut line, blue);
            }
            pop_trailing_space(&mut line);
            body.push_str(&format!("{}\n", line));
        }
        body
    }

    pub fn to_ppm(&self) -> String {
        format!("{}{}", self.generate_header(), self.generate_body())
    }
}

fn scale_colour_value(colour_value: f64, max_colour_value: usize) -> usize {
    let scaled_value = (colour_value * max_colour_value as f64).round() as usize;
    if scaled_value > max_colour_value {
        max_colour_value
    } else {
        scaled_value
    }
}
fn pop_trailing_space(line: &mut String) {
    if line.ends_with(' ') {
        line.pop();
    }
}

fn add_to_line_or_body(body: &mut String, line: &mut String, value: usize) {
    if line.len() + value.to_string().len() > 70 {
        pop_trailing_space(line);
        body.push_str(&format!("{}\n", line));
        line.clear();
    }

    line.push_str(&format!("{} ", value));
}

#[cfg(test)]
mod tests {
    use crate::ppm_wrapper::{scale_colour_value, PpmWrapper};
    use crate::Canvas;
    use core::Colour;

    fn fill_canvas_with_colour(canvas: &mut Canvas, colour: Colour) {
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                canvas.write_pixel(x, y, colour.clone()).unwrap();
            }
        }
    }

    #[test]
    fn constructing_the_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm_wrapper = PpmWrapper::new(canvas, 255);
        assert_eq!(ppm_wrapper.generate_header(), "P3\n5 3\n255\n");
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Colour::new(1.5, 0.0, 0.0);
        let c2 = Colour::new(0.0, 0.5, 0.0);
        let c3 = Colour::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, c1).unwrap();
        canvas.write_pixel(2, 1, c2).unwrap();
        canvas.write_pixel(4, 2, c3).unwrap();
        assert_eq!(
            PpmWrapper::new(canvas, 255).generate_body(),
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
             0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
             0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_pixel_data() {
        let mut canvas = Canvas::new(10, 2);
        fill_canvas_with_colour(&mut canvas, Colour::new(1.0, 0.8, 0.6));
        assert_eq!(
            PpmWrapper::new(canvas, 255).generate_body(),
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
             153 255 204 153 255 204 153 255 204 153 255 204 153\n\
             255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
             153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let canvas = Canvas::new(5, 3);
        assert_eq!(
            PpmWrapper::new(canvas, 255)
                .generate_body()
                .chars()
                .last()
                .unwrap(),
            '\n'
        );
    }

    #[test]
    fn ppm_header_and_body() {
        let mut canvas = Canvas::new(10, 2);
        fill_canvas_with_colour(&mut canvas, Colour::new(1.0, 0.8, 0.6));
        assert_eq!(
            PpmWrapper::new(canvas, 255).to_ppm(),
            "P3\n10 2\n255\n\
             255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
             153 255 204 153 255 204 153 255 204 153 255 204 153\n\
             255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
             153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        )
    }

    #[test]
    fn test_scale_colour_value() {
        assert_eq!(scale_colour_value(1.0, 255), 255);
        assert_eq!(scale_colour_value(0.5, 255), 128);
        assert_eq!(scale_colour_value(0.25, 255), 64);
        assert_eq!(scale_colour_value(0.0, 255), 0);
        assert_eq!(scale_colour_value(-1.0, 255), 0);
        assert_eq!(scale_colour_value(1.5, 255), 255);
    }
}
