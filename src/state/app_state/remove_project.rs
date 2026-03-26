use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
};
use std::{
    fs::{read_to_string, remove_dir_all, remove_file, write},
    path::PathBuf,
};

impl AppState
{
    pub fn remove_project(&mut self) -> Result<(), Error>
    {
        if self.selected_project.is_none()
        {
            return Ok(());
        }

        let projects_path: PathBuf = AppState::get_config_dir(String::from("projects.json"), None)?;

        if let Some(index) = self.selected_project
            && self.project_list[index].exists
        {
            if let Err(err) = remove_file(self.project_list[index].path.join(".projman"))
            {
                return Err(error!(Error::Remove, ".projman file", err));
            }

            if self.delete_project_folder
                && let Err(err) = remove_dir_all(&self.project_list[index].path)
            {
                return Err(error!(Error::Remove, "project folder", err));
            }
        }

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

        if let Some(index) = self.selected_project
        {
            projects.remove(index);
        }

        let projects_to_json: String = match serde_json::to_string_pretty(&projects)
        {
            Ok(string) => string,
            Err(err) =>
            {
                return Err(error!(Error::Read, "projects.json", err));
            }
        };

        if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
        {
            return Err(error!(Error::Write, "projects.json", err));
        };

        if let Some(index) = self.selected_project
        {
            self.project_list.remove(index);
        }

        Ok(())
    }
}
