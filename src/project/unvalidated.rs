use crate::{
    prelude::*,
    project::{Project, Unvalidated},
};
use std::fs;

impl Project<Unvalidated> {
    pub fn exists(&self) -> Result<bool> {
        let path = &self.path;

        if !path.is_dir() {
            return Ok(false);
        }

        let project_file_path = self.get_project_file_path();

        if !project_file_path.is_file() {
            return Ok(false);
        }

        let project = toml::from_str::<Self>(
            &fs::read_to_string(project_file_path)
                .map_err(|err| Error::ReadProjectFile(err.to_string()))?,
        )
        .map_err(|err| Error::ReadProjectFile(err.to_string()))?;

        if project.name != self.name {
            return Ok(false);
        }

        if project.template_name != self.template_name {
            return Ok(false);
        }

        if project.repo != self.repo {
            return Ok(false);
        }

        if project.license != self.license {
            return Ok(false);
        }

        Ok(true)
    }
}
