use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiskinConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub filters: FiltersConfig,
    #[serde(default)]
    pub caveman: CavemanConfig,
    #[serde(default)]
    pub analytics: AnalyticsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_false")]
    pub ultra_compact: bool,
    #[serde(default)]
    pub exclude_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiltersConfig {
    #[serde(default)]
    pub max_lines: usize,
    #[serde(default = "default_false")]
    pub strip_comments: bool,
    #[serde(default = "default_true")]
    pub deduplicate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CavemanConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_level")]
    pub level: CavemanLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CavemanLevel {
    Lite,
    Full,
    Ultra,
    Aggressive,
}

impl std::fmt::Display for CavemanLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CavemanLevel::Lite => write!(f, "lite"),
            CavemanLevel::Full => write!(f, "full"),
            CavemanLevel::Ultra => write!(f, "ultra"),
            CavemanLevel::Aggressive => write!(f, "aggressive"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
}

impl Default for MiskinConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            filters: FiltersConfig::default(),
            caveman: CavemanConfig::default(),
            analytics: AnalyticsConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ultra_compact: false,
            exclude_commands: vec![],
        }
    }
}

impl Default for FiltersConfig {
    fn default() -> Self {
        Self {
            max_lines: 200,
            strip_comments: false,
            deduplicate: true,
        }
    }
}

impl Default for CavemanConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            level: CavemanLevel::Full,
        }
    }
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            data_dir: default_data_dir(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_level() -> CavemanLevel {
    CavemanLevel::Full
}

fn default_data_dir() -> PathBuf {
    let dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("miskin");
    dir
}

impl MiskinConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path();
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("miskin")
            .join("config.toml")
    }
}
