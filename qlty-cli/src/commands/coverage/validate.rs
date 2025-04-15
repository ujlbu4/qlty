use crate::{CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;
use qlty_config::{QltyConfig, Workspace};
use qlty_coverage::formats::Formats;
use qlty_coverage::publish::{Planner, Reader, Settings};
use serde::Serialize;
use serde_json;
use std::path::PathBuf;

const DEFAULT_THRESHOLD: f64 = 90.0;

#[derive(Debug, Args)]
pub struct Validate {
    #[arg(long, value_enum)]
    /// The format of the coverage report to transform. If not specified, the format will be inferred from the file extension or contents.
    pub report_format: Option<Formats>,

    #[arg(long, hide = true)]
    pub output_dir: Option<PathBuf>,

    #[arg(long)]
    /// The prefix to add to file paths in coverage payloads, to make them match the project's directory structure.
    pub add_prefix: Option<String>,

    #[arg(long)]
    /// The prefix to remove from absolute paths in coverage payloads to make them relative to the project root.
    /// This is usually the directory in which the tests were run. Defaults to the root of the git repository.
    pub strip_prefix: Option<String>,

    #[arg(long)]
    /// The minimum percentage of coverage report files to match the file system for validation to succeed.
    /// If not specified, defaults to 90%.
    pub threshold: Option<f64>,

    // Path to zip file
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq)]
enum Status {
    #[default]
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Debug, Clone, Serialize, Default)]
struct ValidationResult {
    files_present: usize,
    files_missing: usize,
    total_files: usize,
    coverage_percentage: f64,
    threshold: f64,
    status: Status,
}

impl Validate {
    pub fn execute(&self, _args: &crate::Arguments) -> Result<CommandSuccess, CommandError> {
        let plan = Planner::new(
            &Self::load_config(),
            &Settings {
                add_prefix: self.add_prefix.clone(),
                strip_prefix: self.strip_prefix.clone(),
                report_format: self.report_format,
                paths: vec![self.path.clone()],
                zip_file: true,
                ..Default::default()
            },
        )
        .compute()?;

        let results = Reader::new(&plan).read()?;

        let mut validation_result = ValidationResult {
            ..Default::default()
        };

        results.file_coverages.iter().for_each(|file| {
            if PathBuf::from(&file.path).exists() {
                validation_result.files_present += 1;
            } else {
                validation_result.files_missing += 1;
            }
        });

        let total_files = validation_result.files_present + validation_result.files_missing;
        let threshold = self.threshold.unwrap_or(DEFAULT_THRESHOLD);
        let coverage_percentage =
            (validation_result.files_present as f64 / total_files as f64) * 100.0;

        if coverage_percentage < threshold {
            validation_result.status = Status::Invalid;
        } else {
            validation_result.status = Status::Valid;
        }

        validation_result.total_files = total_files;
        validation_result.coverage_percentage = coverage_percentage;
        validation_result.threshold = threshold;

        println!("{}", serde_json::to_string_pretty(&validation_result)?);

        if validation_result.status == Status::Invalid {
            return Err(CommandError::CoverageValidation {
                message: format!(
                    "Only {}% of the files are present on the filesystem. Threshold is set to {}%",
                    validation_result.coverage_percentage, validation_result.threshold
                ),
            });
        }

        CommandSuccess::ok()
    }

    fn load_config() -> QltyConfig {
        Workspace::new()
            .and_then(|workspace| workspace.config())
            .unwrap_or_default()
    }
}
