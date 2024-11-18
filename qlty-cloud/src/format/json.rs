use super::Formatter;
use serde::Serialize;
use std::io::Write;

#[derive(Debug)]
pub struct JsonFormatter<T: Serialize> {
    data: T,
}

impl<T: Serialize + 'static> JsonFormatter<T> {
    pub fn new(data: T) -> Box<dyn Formatter> {
        Box::new(Self { data })
    }
}

impl<T: Serialize> Formatter for JsonFormatter<T> {
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self.data)?;
        writer.write_all(json.as_bytes())?;
        writer.write_all(b"\n")?;
        Ok(())
    }
}
