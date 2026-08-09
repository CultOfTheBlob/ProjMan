use crate::{config_dir::ConfigDir, prelude::*, theme::ThemeType};
use gpui::Global;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct General {
    pub projects_dir: String,
    pub delete_project_folder: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub theme: ThemeType,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub general: General,
    pub theme: Theme,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config = Self::read_config_file()?;
        config.validate()?;

        Ok(config)
    }

    fn read_config_file() -> Result<Self> {
        let default_config_contents = toml::to_string_pretty(&Self::default())
            .map_err(|err| Error::ParseConfig(err.to_string()))?;

        let config_path =
            ConfigDir::ConfigFile.get_file(Some(&default_config_contents))?;

        let config_string = fs::read_to_string(&config_path)
            .map_err(|err| Error::ParseConfig(err.to_string()))?;

        toml::from_str::<Self>(&config_string)
            .map_err(|err| Error::ParseConfig(err.to_string()))
    }

    fn validate(&self) -> Result<()> {
        if self.general.projects_dir.is_empty() {
            return Err(Error::ValidateConfig(String::from(
                "project_dir field is empty!",
            )));
        }

        if !PathBuf::from(&self.general.projects_dir).is_dir() {
            return Err(Error::ValidateConfig(String::from(
                "project_dir is not a directory!",
            )));
        }

        Ok(())
    }
}

impl Global for Config {}
