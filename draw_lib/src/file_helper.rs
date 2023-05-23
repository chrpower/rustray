use crate::ppm_wrapper::PpmWrapper;
use std::io;

pub trait Writable {
    fn open(filename: String) -> io::Result<Self>
    where
        Self: Sized;
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
}

impl Writable for std::fs::File {
    fn open(filename: String) -> io::Result<Self> {
        std::fs::File::create(filename)
    }
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        io::Write::write_all(self, buf)
    }
}

#[allow(dead_code)]
fn generate_default_filename(p: &PpmWrapper) -> String {
    use chrono::Local;
    format!(
        "{}_{}_{}.ppm",
        p.width(),
        p.height(),
        Local::now().format("%Y%m%d%H%M%S")
    )
}

#[allow(dead_code)]
pub fn save_ppm_to_file<W: Writable>(
    ppm_wrapper: &PpmWrapper,
    filename: Option<String>,
) -> io::Result<()> {
    let filename = match filename {
        Some(name) => name,
        None => generate_default_filename(ppm_wrapper),
    };

    let mut file = W::open(filename)?;
    file.write_all(ppm_wrapper.to_ppm().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Canvas;
    use crate::ppm_wrapper::PpmWrapper;

    struct MockWritable {
        buffer: Vec<u8>,
    }

    impl Writable for MockWritable {
        fn open(_filename: String) -> io::Result<Self> {
            Ok(MockWritable { buffer: vec![] })
        }
        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
            self.buffer.extend_from_slice(buf);
            Ok(())
        }
    }

    #[test]
    fn test_generate_default_filename() {
        let ppm_wrapper = PpmWrapper::new(Canvas::new(10, 10), 255);
        let filename = generate_default_filename(&ppm_wrapper);
        assert!(filename.starts_with("10_10_"));
        assert!(filename.ends_with(".ppm"));
    }

    #[test]
    fn test_save_ppm_to_file() {
        let ppm_wrapper = PpmWrapper::new(Canvas::new(10, 10), 255);
        let filename = Some("test.ppm".to_string());
        assert!(save_ppm_to_file::<MockWritable>(&ppm_wrapper, filename).is_ok());
    }
}
