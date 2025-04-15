use crate::{utils::fs::path_to_string, WorkspaceEntry};
use qlty_config::config::Ignore;
use std::path::PathBuf;

use super::WorkspaceEntryMatcher;

#[derive(Debug)]
pub struct PluginSpecificIgnoreMatcher {
    plugin_name: String,
    ignores: Vec<Ignore>,
    root: PathBuf,
}

impl PluginSpecificIgnoreMatcher {
    pub fn new(plugin_name: String, ignores: Vec<Ignore>, root: PathBuf) -> Self {
        for ignore in &ignores {
            ignore.initialize_globset();
        }

        Self {
            plugin_name,
            ignores,
            root,
        }
    }
}

impl WorkspaceEntryMatcher for PluginSpecificIgnoreMatcher {
    fn matches(&self, entry: WorkspaceEntry) -> Option<WorkspaceEntry> {
        let path_str = path_to_string(entry.path.strip_prefix(&self.root).unwrap_or(&entry.path));

        if self.ignores.iter().any(|ignore| {
            ignore.plugins.contains(&self.plugin_name) && ignore.matches_path(&path_str)
        }) {
            None
        } else {
            Some(entry)
        }
    }
}
