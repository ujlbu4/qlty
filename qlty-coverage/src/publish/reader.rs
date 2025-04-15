use crate::formats::{parser_for, Formats};
use crate::publish::{Plan, Results};
use anyhow::bail;
use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;
use zip::ZipArchive;

#[derive(Debug, Clone)]
pub struct Reader {
    plan: Plan,
}

impl Reader {
    pub fn new(plan: &Plan) -> Self {
        Self {
            plan: plan.to_owned(),
        }
    }

    pub fn read(&self) -> Result<Results> {
        let mut file_coverages = vec![];
        let mut report_files = vec![];

        if self.plan.zip_file {
            return Self::read_zip_file(self.plan.report_files[0].path.clone());
        }

        for report_file in &self.plan.report_files {
            let path = PathBuf::from(&report_file.path);

            if !path.exists() {
                bail!("Coverage report file not found: {}", path.display());
            }

            let format = Formats::from_str(&report_file.format)?;
            let parser = parser_for(&format);

            let coverages = parser.parse_file(&path)?;
            file_coverages.extend(coverages);
            report_files.push(report_file.clone());
        }

        Ok(Results {
            report_files,
            file_coverages,
        })
    }

    fn read_zip_file(path: String) -> Result<Results> {
        let mut file_coverages = vec![];
        let zip_file = PathBuf::from(&path);

        if !zip_file.exists() {
            bail!("Coverage report file not found: {}", zip_file.display());
        }

        if zip_file.extension().is_some_and(|ext| ext == "zip") {
            let file = File::open(&zip_file)?;
            let mut archive = ZipArchive::new(BufReader::new(file))?;

            if let Ok(coverage_zip) = archive.by_name("file_coverages.jsonl") {
                let mut reader = BufReader::new(coverage_zip);
                let mut file_contents = String::new();
                reader.read_to_string(&mut file_contents)?;

                let parser = parser_for(&Formats::Qlty);
                let coverages = parser.parse_text(&file_contents)?;
                file_coverages.extend(coverages);
            } else {
                return Err(anyhow::anyhow!(
                    "file_coverages.jsonl not found in the zip file"
                ));
            }

            Ok(Results {
                report_files: vec![],
                file_coverages,
            })
        } else {
            Err(anyhow::anyhow!(
                "Expected zip file got {}",
                zip_file.display()
            ))
        }
    }
}
