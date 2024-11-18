use crate::Parser;
use anyhow::{Context, Result};
use qlty_types::tests::v1::FileCoverage;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "coverage")]
struct CloverSource {
    project: Project,
}

#[derive(Debug, Deserialize)]
struct Project {
    file: Vec<File>,
}

#[derive(Debug, Deserialize, Clone)]
struct File {
    name: String,
    line: Option<Vec<Line>>,
    metrics: Metrics,
}

#[derive(Debug, Deserialize, Clone)]
struct Metrics {
    loc: i64,
}

#[derive(Debug, Deserialize, Clone)]
struct Line {
    num: i64,
    count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Clover {}

impl Clover {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parser for Clover {
    fn parse_text(&self, text: &str) -> Result<Vec<FileCoverage>> {
        let source: CloverSource =
            serde_xml_rs::from_str(text).with_context(|| "Failed to parse XML text")?;

        let mut file_coverages = vec![];

        for file in source.project.file.iter() {
            let mut line_hits = Vec::new();

            if let Some(ref lines) = file.line {
                for line in lines.iter() {
                    for _x in (line_hits.len() as i64)..(line.num - 1) {
                        line_hits.push(-1);
                    }

                    line_hits.push(line.count);
                }
            }

            for _x in (line_hits.len() as i64)..file.metrics.loc {
                line_hits.push(-1);
            }

            let file_coverage = FileCoverage {
                path: file.name.clone(),
                hits: line_hits,
                ..Default::default()
            };

            file_coverages.push(file_coverage);
        }

        Ok(file_coverages)
    }
}
