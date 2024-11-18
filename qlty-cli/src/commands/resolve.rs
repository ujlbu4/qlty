use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;
use qlty_skills::Resolver;

const MAX_MAX_LOOPS: usize = 20;

#[derive(Args, Debug)]
pub struct Resolve {
    #[arg(long)]
    pub filter: String,

    #[arg(long)]
    /// Stop after this many fixes
    pub max_fixes: Option<usize>,

    #[arg(long)]
    /// Maximum number of fix iterations
    pub max_loops: Option<usize>,

    /// Files to resolve
    pub paths: Vec<String>,
}

impl Resolve {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        let resolver = Resolver {
            paths: self.paths.clone(),
            filter: self.filter.clone().into(),
            max_fixes: self.max_fixes.unwrap_or(1_000),
            max_loops: self.max_loops.unwrap_or(MAX_MAX_LOOPS),
        };

        resolver.execute()?;

        CommandSuccess::ok()
    }
}
