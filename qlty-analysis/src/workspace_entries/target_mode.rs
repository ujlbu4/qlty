use crate::git::DiffMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetMode {
    All,
    Sample(usize),
    Paths(usize),
    UpstreamDiff(String),
    HeadDiff,
}

impl Default for TargetMode {
    fn default() -> Self {
        TargetMode::All
    }
}

impl TargetMode {
    pub fn diff_mode(&self) -> DiffMode {
        match self {
            TargetMode::UpstreamDiff(upstream) => DiffMode::UpstreamToWorkdir(upstream.to_string()),
            TargetMode::HeadDiff => DiffMode::HeadToWorkdir,
            _ => panic!("diff_mode() called on {:?}", self),
        }
    }
}
