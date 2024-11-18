use qlty_config::config::issue_transformer::IssueTransformer;
use qlty_types::analysis::v1::{Issue, Level};

#[derive(Debug, Clone)]
pub struct LevelFilter {
    pub level: Level,
}

impl IssueTransformer for LevelFilter {
    fn transform(&self, issue: Issue) -> Option<Issue> {
        if issue.level >= self.level.into() {
            Some(issue)
        } else {
            None
        }
    }

    fn clone_box(&self) -> Box<dyn IssueTransformer> {
        Box::new(self.clone())
    }
}
