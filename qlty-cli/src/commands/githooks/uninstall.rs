use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::Result;
use clap::Args;
use qlty_config::Workspace;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[derive(Args, Debug)]
pub struct Uninstall {}

impl Uninstall {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        Workspace::require_initialized()?;
        let hooks = [".git/hooks/pre-commit", ".git/hooks/pre-push"];

        for hook in &hooks {
            let metadata = fs::metadata(hook)?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(permissions.mode() & !0o111); // Remove execute permissions
            fs::set_permissions(hook, permissions)?;
        }

        CommandSuccess::ok()
    }
}
