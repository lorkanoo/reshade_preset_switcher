mod notifications;
pub mod preset_rule;
pub mod reshade_config;

use crate::addon::{Addon, VERSION};
use crate::config::notifications::Notifications;
use crate::config::preset_rule::PresetRule;
pub use crate::config::reshade_config::ReshadeConfig;
use log::info;
use nexus::paths::{get_addon_dir, get_game_dir};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::MutexGuard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default = "Notifications::default")]
    pub notifications: Notifications,
    pub preset_rules: Vec<PresetRule>,
    pub reshade: ReshadeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            notifications: Notifications::default(),
            preset_rules: Vec::new(),
            reshade: ReshadeConfig::default(),
        }
    }
}

impl Config {
    pub fn try_load() -> Option<Self> {
        let path = Self::file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read config: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse config: {err}"))
            .ok()?;
        info!("Loaded config from \"{}\"", path.display());
        Some(config)
    }

    pub fn save(&self) {
        let path = Self::file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &self).expect("failed to serialize config");
                info!("Saved config to \"{}\"", path.display())
            }
            Err(err) => log::error!("Failed to save config: {err}"),
        }
    }

    pub fn file() -> PathBuf {
        config_dir().join("config.json")
    }

    pub fn valid(&self) -> bool {
        self.reshade.ini_path.exists()
    }
}

pub fn config_dir() -> PathBuf {
    get_addon_dir("reshade_preset_switcher").expect("invalid config directory")
}

pub fn game_dir() -> PathBuf {
    get_game_dir().expect("invalid game directory")
}

fn default_version() -> String {
    VERSION.to_string()
}

pub fn migrate_configs(addon: &mut MutexGuard<Addon>) {
    addon.config.version = VERSION.to_string();
}

#[allow(dead_code)]
fn version_older_than(older: &str, than: &str) -> bool {
    Version::parse(older).unwrap() < Version::parse(than).unwrap()
}
