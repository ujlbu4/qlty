use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;
use qlty_cloud::load_auth_token;

#[derive(Args, Debug)]
pub struct Login {}

impl Login {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        load_auth_token()?;
        CommandSuccess::ok()
    }
}
