use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::utils::{Error, Result};

use super::Space;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub spaces: Vec<Space>,
}

// associated functions
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

// methods
impl Config {
    pub fn add_space(&mut self, name: String, path: String) -> Result<()> {
        if !self.spaces.is_empty() {
            if self.spaces.iter().any(|sp| sp.name.contains(&name)) {
                return Err(Error::Generic(String::from(
                    "El nombre ya se encuentra asociado a un espacio",
                )));
            }

            if let Some(space) = self.spaces.iter().find(|sp| sp.path == Path::new(&path)) {
                return Err(Error::Generic(String::from(format!(
                    "El path se encuentra asociado al espacio llamado {}",
                    space.name
                ))));
            }
        }

        let new_space = Space::new_from(name, path)?;

        self.spaces.push(new_space);
        self.save()?;

        Ok(())
    }

    pub fn list_space(&self, name: String) -> Result<()> {
        let found_space = self
            .spaces
            .iter()
            .find(|sp| sp.name == name)
            .ok_or(Error::Generic(String::from("No se encontró el espacio")))?;

        if !found_space.path.exists() {
            return Err(Error::Generic(String::from(
                "Directorio base no se encontró",
            )));
        }

        for entry in WalkDir::new(&found_space.path).max_depth(1) {
            println!("{}", entry?.path().display());
        }

        Ok(())
    }

    fn save(&self) -> Result<()> {
        let cfg_file_path = Self::config_path()?;

        fs::write(cfg_file_path, serde_json::to_string_pretty(self)?)?;

        Ok(())
    }
}
