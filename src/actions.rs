use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::{
    config::{Config, Space},
    utils::{Error, Result},
};

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

    pub fn list_space(self, name: String) -> Result<()> {
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
