use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
};

impl AppState
{
    pub fn import_project(&mut self) -> Result<(), Error>
    {
        let projects_path: PathBuf = AppState::get_config_dir(String::from("projects.json"), None)?;

        let path: PathBuf =
            PathBuf::from(&self.config.general.projects_dir).join(&self.import_project_path);

        let project_from_toml: String = match &read_to_string(path.join("projman.toml"))
        {
            Ok(string) => string.to_string(),
            Err(err) =>
            {
                return Err(error!(Error::Read, "projman.toml", err));
            }
        };

        let project: Project = match toml::from_str(&project_from_toml)
        {
            Ok(project) => project,
            Err(err) => return Err(error!(Error::Parse, "projman.toml", err)),
        };

        let projects_from_json: String = match read_to_string(&projects_path)
        {
            Ok(json) => json,
            Err(err) =>
            {
                return Err(error!(Error::Read, "projects.json", err));
            }
        };

        let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
        {
            Ok(projects) => projects,
            Err(err) =>
            {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        projects.push(project);

        let projects_to_json: String = match serde_json::to_string_pretty(&projects)
        {
            Ok(json) => json,
            Err(err) =>
            {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
        {
            return Err(error!(Error::Write, "projects.json", err));
        };

        Ok(())
    }
}
