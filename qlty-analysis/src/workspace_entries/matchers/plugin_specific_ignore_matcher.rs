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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceEntryKind;
    use std::time::SystemTime;

    #[test]
    fn test_matches_ignores_file_for_specific_plugin() {
        let plugin_name = "test-plugin".to_string();
        let root = PathBuf::from("/project");

        let mut ignore = Ignore::default();
        ignore.file_patterns = vec!["*.log".to_string()];
        ignore.plugins = vec![plugin_name.clone()];

        let matcher =
            PluginSpecificIgnoreMatcher::new(plugin_name.clone(), vec![ignore], root.clone());

        let entry = WorkspaceEntry {
            path: root.join("file.log"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        let result = matcher.matches(entry);

        assert!(result.is_none(), "Expected file.log to be ignored");
    }

    #[test]
    fn test_does_not_ignore_file_for_different_plugin() {
        let plugin_name = "test-plugin".to_string();
        let other_plugin = "other-plugin".to_string();
        let root = PathBuf::from("/project");

        let mut ignore = Ignore::default();
        ignore.file_patterns = vec!["*.log".to_string()];
        ignore.plugins = vec![other_plugin];

        let matcher = PluginSpecificIgnoreMatcher::new(plugin_name, vec![ignore], root.clone());

        let entry = WorkspaceEntry {
            path: root.join("file.log"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        let result = matcher.matches(entry);

        assert!(
            result.is_some(),
            "Expected file.log not to be ignored for a different plugin"
        );
    }

    #[test]
    fn test_does_not_ignore_non_matching_file() {
        let plugin_name = "test-plugin".to_string();
        let root = PathBuf::from("/project");

        let mut ignore = Ignore::default();
        ignore.file_patterns = vec!["*.log".to_string()];
        ignore.plugins = vec![plugin_name.clone()];

        let matcher = PluginSpecificIgnoreMatcher::new(plugin_name, vec![ignore], root.clone());

        let entry = WorkspaceEntry {
            path: root.join("file.txt"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        let result = matcher.matches(entry);

        assert!(result.is_some(), "Expected file.txt not to be ignored");
    }

    #[test]
    fn test_handles_multiple_ignores() {
        let plugin_name = "test-plugin".to_string();
        let root = PathBuf::from("/project");

        let mut ignore1 = Ignore::default();
        ignore1.file_patterns = vec!["*.log".to_string()];
        ignore1.plugins = vec![plugin_name.clone()];

        let mut ignore2 = Ignore::default();
        ignore2.file_patterns = vec!["*.tmp".to_string()];
        ignore2.plugins = vec![plugin_name.clone()];

        let matcher =
            PluginSpecificIgnoreMatcher::new(plugin_name, vec![ignore1, ignore2], root.clone());

        let entry1 = WorkspaceEntry {
            path: root.join("file.log"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        let entry2 = WorkspaceEntry {
            path: root.join("file.tmp"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        let entry3 = WorkspaceEntry {
            path: root.join("file.txt"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        assert!(
            matcher.matches(entry1).is_none(),
            "Expected file.log to be ignored"
        );
        assert!(
            matcher.matches(entry2).is_none(),
            "Expected file.tmp to be ignored"
        );
        assert!(
            matcher.matches(entry3).is_some(),
            "Expected file.txt not to be ignored"
        );
    }

    #[test]
    fn test_correctly_strips_root_prefix() {
        let plugin_name = "test-plugin".to_string();
        let root = PathBuf::from("/project");

        let mut ignore = Ignore::default();
        ignore.file_patterns = vec!["src/*.log".to_string()];
        ignore.plugins = vec![plugin_name.clone()];

        let matcher = PluginSpecificIgnoreMatcher::new(plugin_name, vec![ignore], root.clone());

        let entry_in_src = WorkspaceEntry {
            path: root.join("src/file.log"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        let entry_in_root = WorkspaceEntry {
            path: root.join("file.log"),
            kind: WorkspaceEntryKind::File,
            content_modified: SystemTime::now(),
            contents_size: 100,
            language_name: None,
        };

        assert!(
            matcher.matches(entry_in_src).is_none(),
            "Expected src/file.log to be ignored"
        );
        assert!(
            matcher.matches(entry_in_root).is_some(),
            "Expected root/file.log not to be ignored"
        );
    }
}
