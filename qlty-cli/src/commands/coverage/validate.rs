use crate::{CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;
use qlty_coverage::validate::{ValidationResult, ValidationStatus};
use serde_json;

#[derive(Debug, Args)]
pub struct Validate {
    #[arg(long)]
    /// The minimum percentage of coverage report files to match the file system for validation to succeed.
    /// If not specified, defaults to 90%.
    pub path_threshold: Option<f64>,

    // Path to zip file
    pub path: String,

    // Print the result in JSON format
    #[arg(long, default_value_t = false)]
    pub json: bool,
}

impl Validate {
    pub fn execute(&self, _args: &crate::Arguments) -> Result<CommandSuccess, CommandError> {
        let validation_result = ValidationResult::compute(&self.path, self.path_threshold)?;

        if self.json {
            println!("{}", serde_json::to_string_pretty(&validation_result)?);
        }

        match validation_result.status {
            ValidationStatus::Valid => CommandSuccess::ok(),
            ValidationStatus::Invalid => Err(CommandError::CoverageValidation {
                message: format!(
                    "Only {}% of the files are present on the filesystem. Threshold is set to {}%",
                    validation_result.coverage_percentage, validation_result.threshold
                ),
            }),
            ValidationStatus::NoCoverageData => Err(CommandError::CoverageValidation {
                message: "No coverage data found".to_string(),
            }),
        }
    }
}
