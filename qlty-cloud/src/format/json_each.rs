use super::Formatter;
use serde::Serialize;
use std::io::Write;

#[derive(Debug)]
pub struct JsonEachRowFormatter<T: Serialize> {
    data: Vec<T>, // Assuming data is a collection of rows
}

impl<T: Serialize + 'static> JsonEachRowFormatter<T> {
    pub fn new(data: Vec<T>) -> Box<dyn Formatter> {
        Box::new(Self { data })
    }
}

impl<T: Serialize> Formatter for JsonEachRowFormatter<T> {
    fn write_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        for row in &self.data {
            let json_row = serde_json::to_string(&row)?;
            writer.write_all(json_row.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        Ok(())
    }
}
