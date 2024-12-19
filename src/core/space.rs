use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Space {
    pub name: String,
    pub short: String,
    pub path: PathBuf,
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