use crate::publish::{Plan, Report, Results};
use anyhow::Result;
use qlty_types::tests::v1::FileCoverage;

pub struct Processor {
    plan: Plan,
    results: Results,
}

impl Processor {
    pub fn new(plan: &Plan, results: Results) -> Self {
        Self {
            plan: plan.clone(),
            results,
        }
    }

    pub fn compute(&self) -> Result<Report> {
        let mut report_files = self.results.report_files.clone();

        report_files.iter_mut().for_each(|f| {
            f.build_id = self.plan.metadata.build_id.clone();
            f.tag = self.plan.metadata.tag.clone();
        });

        Ok(Report {
            metadata: self.plan.metadata.clone(),
            report_files,
            file_coverages: self
                .results
                .file_coverages
                .iter()
                .filter_map(|file_coverage| self.transform(file_coverage.to_owned()))
                .collect::<Vec<_>>(),
        })
    }

    fn transform(&self, file_coverage: FileCoverage) -> Option<FileCoverage> {
        let mut file_coverage: Option<FileCoverage> = Some(file_coverage.clone());

        for transformer in self.plan.transformers.iter() {
            if file_coverage.is_some() {
                file_coverage = transformer.transform(file_coverage.unwrap());
            } else {
                return None;
            }
        }

        file_coverage
    }
}
