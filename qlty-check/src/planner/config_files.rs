use super::{config::enabled_plugins, Planner};
use anyhow::{Context, Result};
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use qlty_config::config::Exclude;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tracing::{debug, error};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct PluginConfigFile {
    pub path: PathBuf,
    pub contents: String,
}

impl Ord for PluginConfigFile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for PluginConfigFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PluginConfigFile {
    pub fn from_path(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file from path {}", path.display()))?;

        Ok(Self {
            path: path.to_path_buf(),
            contents,
        })
    }
}

#[derive(Debug, Clone)]
struct PluginConfig {
    plugin_name: String,
    config_globset: GlobSet,
}

pub fn config_globset(config_files: &Vec<PathBuf>) -> Result<GlobSet> {
    let mut globset = GlobSetBuilder::new();

    for config_file in config_files {
        let glob = GlobBuilder::new(
            config_file
                .to_str()
                .ok_or(anyhow::anyhow!("Invalid path: {:?}", config_file))?,
        )
        .literal_separator(true)
        .build()?;

        globset.add(glob);
    }

    Ok(globset.build()?)
}

fn exclude_globset(exclude: &Vec<Exclude>) -> Result<GlobSet> {
    let mut globset = GlobSetBuilder::new();

    for exclude in exclude {
        for pattern in &exclude.file_patterns {
            let glob = GlobBuilder::new(pattern)
                .literal_separator(true)
                .build()
                .with_context(|| format!("Failed to build glob for pattern: {}", pattern))?;

            globset.add(glob);
        }
    }

    Ok(globset.build()?)
}

pub fn plugin_configs(planner: &Planner) -> Result<HashMap<String, Vec<PluginConfigFile>>> {
    let plugins = enabled_plugins(planner)?;
    let mut plugins_configs = vec![];

    for active_plugin in &plugins {
        plugins_configs.push(PluginConfig {
            plugin_name: active_plugin.name.clone(),
            config_globset: config_globset(&active_plugin.plugin.config_files)?,
        });
    }

    let mut configs: HashMap<String, Vec<PluginConfigFile>> = HashMap::new();
    let exclude_globset = exclude_globset(&planner.config.exclude)?;

    for entry in planner.workspace.walker() {
        let entry = entry?;
        if let Some(os_str) = entry.path().file_name() {
            let file_name = os_str.to_os_string();
            for plugin_config in &plugins_configs {
                // Why do we have exclude_globset.is_match(entry.path()) here?!
                // If you want to exclude the file, you should not add it to the configs.
                if plugin_config.config_globset.is_match(&file_name)
                    && !exclude_globset.is_match(entry.path())
                {
                    let entry_path = entry.path();
                    let config_file = match PluginConfigFile::from_path(entry_path) {
                        Ok(config_file) => config_file,
                        _ => {
                            error!("Failed to read config file from path {:?}", entry_path);
                            continue;
                        }
                    };

                    debug!(
                        "Found config file for plugin {}: {:?}",
                        &plugin_config.plugin_name, &config_file.path
                    );
                    configs
                        .entry(plugin_config.plugin_name.clone())
                        .or_default()
                        .push(config_file);
                }
            }
        }
    }

    Ok(configs)
}
