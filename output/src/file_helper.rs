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

fn generate_filename(p: &PpmWrapper) -> String {
    use std::time::SystemTime;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to obtain system time")
        .as_secs();

    format!("{}_{}_{}.ppm", p.width(), p.height(), timestamp)
}

pub fn write_ppm<W: Writable>(ppm_wrapper: &PpmWrapper) -> io::Result<()> {
    let mut file = W::open(generate_filename(ppm_wrapper))?;
    file.write_all(ppm_wrapper.to_ppm().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Canvas;
    use crate::PpmWrapper;
    use std::io;

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
    fn filename() {
        let ppm_wrapper = PpmWrapper::new(Canvas::new(10, 10), 255);
        let filename = generate_filename(&ppm_wrapper);
        assert!(filename.starts_with("10_10_"));
        assert!(filename.ends_with(".ppm"));
    }

    #[test]
    fn write_ppm_to_mock() {
        assert!(write_ppm::<MockWritable>(&PpmWrapper::new(Canvas::new(10, 10), 255)).is_ok());
    }
}
