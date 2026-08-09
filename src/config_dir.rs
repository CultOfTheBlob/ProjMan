use crate::prelude::*;
use directories::ProjectDirs;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub enum ConfigDir {
    ConfigFile,
    Template(String),
    Templates,
    Projects,
}

impl ConfigDir {
    pub fn get_file(self, create_if_missing: Option<&str>) -> Result<PathBuf> {
        let config_path = ProjectDirs::from("", "", "projman")
            .map_or(Err(Error::GetConfigDir), |project_dirs: ProjectDirs| {
                Ok(project_dirs.config_dir().to_path_buf())
            })?;

        fs::create_dir_all(&config_path).map_err(|err| {
            Error::CreateDir(config_path.to_string_lossy().into_owned(), err.to_string())
        })?;

        let path = config_path.join(self.to_string());

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                Error::CreateDir(parent.to_string_lossy().into_owned(), err.to_string())
            })?;
        }

        if path.exists() {
            return Ok(path);
        }

        if let Some(content) = create_if_missing {
            fs::write(&path, content.as_bytes())
                .map_err(|err| Error::CreateDir(self.to_string(), err.to_string()))?;
        }

        Ok(path)
    }
}

impl Display for ConfigDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let string = match self {
            Self::ConfigFile => "config.toml",
            Self::Template(name) => &format!("templates/{name}"),
            Self::Templates => "templates",
            Self::Projects => "projects.yaml",
        };

        write!(f, "{string}")
    }
}
