use std::fs;

use crate::{
    config_dir::ConfigDir,
    prelude::*,
    project::{Project, Unvalidated, valid_project::ValidProject},
};

pub fn load_projects() -> Result<Vec<ValidProject>> {
    let projects_path = ConfigDir::Projects.get_file(Some(""))?;

    let projects = serde_yaml::from_str::<Vec<Project<Unvalidated>>>(
        &fs::read_to_string(&projects_path)
            .map_err(|err| Error::ReadProjectList(err.to_string()))?,
    )
    .map_err(|err| Error::ReadProjectList(err.to_string()))?;

    Ok(projects.into_iter().map(Project::validate).collect())
}
