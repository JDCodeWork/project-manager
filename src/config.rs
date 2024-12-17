use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::utils::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub spaces: Vec<Space>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Space {
    pub name: String,
    pub short: String,
    pub path: PathBuf,
}

impl Config {
    fn default() -> Self {
        Self { spaces: vec![] }
    }

    pub fn start() -> Result<Self> {
        let cfg_file_path = Self::config_path()?;

        if !cfg_file_path.exists() | !cfg_file_path.is_file() {
            Self::new_from_path(&cfg_file_path)?;
        }

        Ok(Self::read_from_path(cfg_file_path)?)
    }

    fn new_from_path(file_path: &PathBuf) -> Result<()> {
        let base_path = file_path
            .parent()
            .ok_or(Error::Generic(String::from("Config path not found")))?;

        if !base_path.exists() | !base_path.is_dir() {
            fs::create_dir_all(base_path)?;
        }

        let default_cfg = serde_json::to_string_pretty(&Config::default())?;

        fs::write(&file_path, default_cfg)?;

        Ok(())
    }

    fn read_from_path(file_path: PathBuf) -> Result<Self> {
        Ok(serde_json::from_str(&fs::read_to_string(file_path)?)?)
    }

    pub fn config_path() -> Result<PathBuf> {
        let base_dirs = directories::BaseDirs::new()
            .ok_or(Error::Generic(String::from("Base directory not found")))?;

        let cfg_file_path = base_dirs
            .config_dir()
            .join("project-manager")
            .join("cfg.json");

        Ok(cfg_file_path)
    }
}

impl Space {
    pub fn new_from(name: String, raw_path: String) -> Result<Self> {
        let short = match name.chars().next() {
            Some(value) => value.to_string(),
            None => String::from(""),
        };

        let path = Path::new(&raw_path).to_path_buf();

        if !path.exists() {
            return Err(Error::Generic(String::from(
                "El path ingresado no se logro encontrar",
            )));
        }

        Ok(Self { name, path, short })
    }
}
