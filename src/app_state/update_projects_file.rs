use crate::{config_dir::ConfigDir, prelude::*, project::valid_project::ValidProject};
use std::fs::File;

pub fn update_projects_file(projects: &Vec<ValidProject>) -> Result<()> {
    let projects_path = ConfigDir::Projects.get_file(Some(""))?;
    let writer = File::create(projects_path)
        .map_err(|err| Error::UpdateProjects(err.to_string()))?;

    serde_yaml::to_writer(writer, projects)
        .map_err(|err| Error::UpdateProjects(err.to_string()))?;

    Ok(())
}
