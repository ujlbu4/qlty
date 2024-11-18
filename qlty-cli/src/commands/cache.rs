use crate::{CommandError, CommandSuccess};
use anyhow::Result;
use clap::{Args, Subcommand};

mod clean;
mod dir;
mod status;

pub use clean::Clean;
pub use dir::Dir;
pub use status::Status;

#[derive(Debug, Args)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]

pub enum Commands {
    /// Delete the entire cache
    Clean(Clean),

    /// Print the cache directory
    Dir(Dir),

    /// Print the status of the cache directory
    Status(Status),
}

impl Arguments {
    pub fn execute(&self, args: &crate::Arguments) -> Result<CommandSuccess, CommandError> {
        match &self.command {
            Commands::Clean(command) => command.execute(args),
            Commands::Dir(command) => command.execute(args),
            Commands::Status(command) => command.execute(args),
        }
    }
}
