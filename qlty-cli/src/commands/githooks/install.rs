use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct Install {}

impl Install {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        CommandSuccess::ok()
    }
}
