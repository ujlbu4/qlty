use crate::{Arguments, CommandError, CommandSuccess};
use anyhow::{Context as _, Result};
use clap::Args;
use qlty_config::Workspace;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt as _};
use std::path::Path;

#[derive(Args, Debug)]
pub struct Install {}

const PRE_COMMIT_HOOK: &str = r#"#!/bin/sh
qlty fmt --trigger pre-commit --index-file="$GIT_INDEX_FILE"
"#;

// const PRE_PUSH_HOOK: &str = r#"#!/bin/sh
// qlty check --trigger pre-push --upstream-from-pre-push --no-formatters --skip-errored-plugins
// "#;

impl Install {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        Workspace::require_initialized()?;

        let qlty_hooks_dir = Path::new(".qlty/hooks");
        fs::create_dir_all(qlty_hooks_dir)?;

        let pre_commit_hook_path = qlty_hooks_dir.join("pre-commit.sh");
        fs::write(&pre_commit_hook_path, PRE_COMMIT_HOOK).with_context(|| {
            format!(
                "Failed to write pre-commit hook to {}",
                pre_commit_hook_path.display()
            )
        })?;

        let git_hooks_dir = Path::new(".git/hooks");
        let pre_commit_symlink = git_hooks_dir.join("pre-commit");

        if pre_commit_symlink.exists() {
            fs::remove_file(&pre_commit_symlink).with_context(|| {
                format!(
                    "Failed to remove existing pre-commit symlink at {}",
                    pre_commit_symlink.display()
                )
            })?;
        }

        let pre_comit_hook_relative_path = Path::new("../../.qlty/hooks/pre-commit.sh");
        symlink(&pre_comit_hook_relative_path, &pre_commit_symlink).with_context(|| {
            format!(
                "Failed to create symlink from {} to {}",
                pre_comit_hook_relative_path.display(),
                pre_commit_symlink.display()
            )
        })?;

        let metadata = fs::metadata(&pre_commit_symlink).with_context(|| {
            format!(
                "Failed to get metadata for pre-commit symlink at {}",
                pre_commit_symlink.display()
            )
        })?;

        let mut perms = metadata.permissions();
        perms.set_mode(0o755);

        fs::set_permissions(&pre_commit_symlink, perms).with_context(|| {
            format!(
                "Failed to set permissions on pre-commit symlink at {}",
                pre_commit_symlink.display()
            )
        })?;

        CommandSuccess::ok()
    }
}
