use qlty_config::config::{DriverDef, PluginDef};

#[derive(Debug, Default, Clone)]
pub struct DriverCandidate {
    pub key: String,
    pub version: String,
    pub name: String,
    pub driver: DriverDef,
}

impl DriverCandidate {
    pub fn build_driver_candidates(plugin_def: &PluginDef) -> Vec<DriverCandidate> {
        let mut driver_candidates = vec![];

        for (driver_name, driver) in plugin_def.drivers.iter() {
            if driver.version.is_empty() {
                driver_candidates.push(DriverCandidate {
                    key: driver_name.clone(),
                    name: driver_name.clone(),
                    driver: driver.clone(),
                    version: Self::get_driver_version(driver, plugin_def),
                });
            } else {
                for driver_version in &driver.version {
                    let version = Self::get_driver_version(driver_version, plugin_def);

                    driver_candidates.push(DriverCandidate {
                        key: format!("{}@{}", driver_name, version),
                        name: driver_name.clone(),
                        version,
                        driver: driver_version.clone(),
                    })
                }
            }
        }

        driver_candidates
    }

    fn get_driver_version(driver: &DriverDef, plugin: &PluginDef) -> String {
        let latest_version = plugin.latest_version.clone();
        let known_good_version = Self::get_known_good_version(driver, plugin);

        if latest_version.as_ref() == known_good_version.as_ref() {
            "latest".to_owned()
        } else {
            known_good_version.unwrap()
        }
    }

    fn get_known_good_version(driver: &DriverDef, plugin: &PluginDef) -> Option<String> {
        if let Some(driver_known_good_version) = &driver.known_good_version {
            Some(driver_known_good_version.clone())
        } else {
            plugin.known_good_version.clone()
        }
    }
}
