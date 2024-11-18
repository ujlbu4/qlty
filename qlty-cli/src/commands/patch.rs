use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;
use console::style;
use qlty_check::{
    executor::staging_area::{Mode, StagingArea},
    patcher::Patcher,
};
use qlty_config::Workspace;
use qlty_types::analysis::v1::Issue;

#[derive(Args, Debug)]
pub struct Patch {
    path: String,
}

impl Patch {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        let contents = std::fs::read_to_string(&self.path)?;
        let issues: Vec<Issue> = serde_json::from_str(&contents)?;

        let workspace = Workspace::new()?;
        let staging_area = StagingArea::generate(Mode::Source, workspace.root.clone(), None);

        let fixed = Patcher::new(&staging_area).try_apply(&issues);

        eprintln!(
            "{}",
            style(format!("✔ {} fixed issues", fixed.len()))
                .green()
                .bold()
        );

        CommandSuccess::ok()
    }
}
