use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
};
use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

impl AppState
{
    pub fn load_projects(&self) -> Result<Vec<Project>, Error>
    {
        let projects_path: PathBuf =
            AppState::get_config_dir(String::from("projects.json"), Some(String::from("[]")))?;

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

        for project in &mut projects
        {
            project.template = self.templates.get(&project.template_name)?;

            let is_dir = project.path.is_dir();

            let has_projman_file = project.path.join("projman.toml").is_file();

            project.exists = is_dir && has_projman_file;
        }

        let projects_to_json: String = match serde_json::to_string_pretty(&projects)
        {
            Ok(json) => json,
            Err(err) =>
            {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        if projects_to_json == projects_from_json
        {
            return Ok(projects);
        }

        if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
        {
            return Err(error!(Error::Write, "projects.json", err));
        };

        Ok(projects)
    }
}
