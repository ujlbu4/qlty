use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct Token {}

impl Token {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        println!(
            "{}",
            qlty_cloud::Token::default().get_with_interactive_prompt()?
        );
        CommandSuccess::ok()
    }
}
