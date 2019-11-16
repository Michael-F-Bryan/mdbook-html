use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename = "kebab-case")]
pub struct Config {
    pub theme: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Config { Config { theme: None } }
}