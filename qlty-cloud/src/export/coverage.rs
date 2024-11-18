use crate::format::{GzFormatter, JsonEachRowFormatter, JsonFormatter};
use anyhow::{Context, Result};
use qlty_types::tests::v1::{CoverageMetadata, FileCoverage, ReportFile};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct CoverageExport {
    pub metadata: CoverageMetadata,
    pub report_files: Vec<ReportFile>,
    pub file_coverages: Vec<FileCoverage>,
    pub to: Option<PathBuf>,
}

impl CoverageExport {
    pub fn export_to(&mut self, directory: Option<PathBuf>) -> Result<()> {
        self.to = Some(directory.unwrap_or_else(|| PathBuf::from("tmp/qlty-coverage")));
        self.export()
    }

    fn export(&self) -> Result<()> {
        let directory = self.to.as_ref().unwrap();

        GzFormatter::new(JsonEachRowFormatter::new(self.report_files.clone()))
            .write_to_file(&directory.join("report_files.json.gz"))?;

        GzFormatter::new(JsonEachRowFormatter::new(self.file_coverages.clone()))
            .write_to_file(&directory.join("file_coverages.json.gz"))?;

        JsonFormatter::new(self.metadata.clone()).write_to_file(&directory.join("metadata.json"))
    }

    pub fn total_size_bytes(&self) -> Result<u64> {
        let mut bytes: u64 = 0;

        bytes += self.read_file("report_files.json.gz")?.len() as u64;
        bytes += self.read_file("file_coverages.json.gz")?.len() as u64;
        bytes += self.read_file("metadata.json")?.len() as u64;

        Ok(bytes)
    }

    pub fn read_file<P: AsRef<Path>>(&self, filename: P) -> Result<Vec<u8>> {
        let path = self.to.as_ref().unwrap().join(filename.as_ref());
        let mut file =
            File::open(&path).with_context(|| format!("Failed to open file: {:?}", path))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        Ok(buffer)
    }
}
