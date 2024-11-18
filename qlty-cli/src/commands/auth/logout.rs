use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct Logout {}

impl Logout {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        let token = qlty_cloud::Token::default();
        token.delete()?;
        CommandSuccess::ok()
    }
}
