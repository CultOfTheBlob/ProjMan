use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
};
use std::fs;

impl AppState
{
    pub fn edit_project(&self) -> Result<(), Error>
    {
        if self.selected_project.is_none()
        {
            return Ok(());
        }

        let projects_path = Self::get_config_dir("projects.json", None)?;

        let projects_from_json = match fs::read_to_string(&projects_path)
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

        if let Some(index) = self.selected_project
        {
            projects[index].name = self.edit_project_name.clone();
            projects[index].repo = self.edit_project_repo.clone();
        }

        let projects_to_json = match serde_json::to_string_pretty(&projects)
        {
            Ok(json) => json,
            Err(err) =>
            {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        if let Err(err) = fs::write(&projects_path, projects_to_json.as_bytes())
        {
            return Err(error!(Error::Write, "projects.json", err));
        }

        Ok(())
    }
}
