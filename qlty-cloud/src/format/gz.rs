use super::Formatter;
use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

pub struct GzFormatter {
    formatter: Box<dyn Formatter>,
}

impl GzFormatter {
    pub fn new(formatter: Box<dyn Formatter>) -> Box<dyn Formatter> {
        Box::new(GzFormatter { formatter })
    }
}

impl Formatter for GzFormatter {
    fn write_to(&self, writer: &mut dyn Write) -> Result<()> {
        let raw_data = self.formatter.read()?;
        let mut encoder = GzEncoder::new(writer, Compression::default());
        encoder.write_all(&raw_data)?;
        encoder.finish()?;
        Ok(())
    }
}
