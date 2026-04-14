use crate::{
    error::{Error, ErrorInfo},
    project::{Project, ProjmanFile},
    state::app_state::AppState,
};
use std::{fs, sync::Arc};

impl AppState
{
    pub fn remove_project(&mut self) -> Result<(), Error>
    {
        if self.selected_project.is_none()
        {
            return Ok(());
        }

        let projects_path = Self::get_config_dir("projects.json", None)?;

        if let Some(index) = self.selected_project
            && self.project_list[index].exists
        {
            if let Err(err) =
                fs::remove_file(self.project_list[index].path.join(ProjmanFile::FILE_NAME))
            {
                return Err(error!(Error::Remove, ProjmanFile::FILE_NAME, err));
            }

            if self.delete_project_folder
                && let Err(err) = fs::remove_dir_all(&self.project_list[index].path)
            {
                return Err(error!(Error::Remove, "project folder", err));
            }
        }

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
            projects.remove(index);
        }

        let projects_to_json = match serde_json::to_string_pretty(&projects)
        {
            Ok(string) => string,
            Err(err) =>
            {
                return Err(error!(Error::Read, "projects.json", err));
            }
        };

        if let Err(err) = fs::write(&projects_path, projects_to_json.as_bytes())
        {
            return Err(error!(Error::Write, "projects.json", err));
        }

        if let Some(index) = self.selected_project
        {
            Arc::make_mut(&mut self.project_list).remove(index);
        }

        Ok(())
    }
}
