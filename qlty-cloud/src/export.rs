pub mod analysis;
pub mod coverage;

pub use analysis::AnalysisExport;
pub use coverage::CoverageExport;

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Protobuf,
}

impl Default for ExportFormat {
    fn default() -> Self {
        ExportFormat::Json
    }
}
