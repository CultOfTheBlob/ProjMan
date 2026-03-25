use std::fs::{read_to_string, write};

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
};

pub fn edit_project(state: &mut AppState) -> Result<(), Error>
{
    if state.selected_project.is_none()
    {
        return Ok(());
    }

    match AppState::get_config_dir(String::from("projects.json"), None)
    {
        Ok(projects_path) =>
        {
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

            if let Some(index) = state.selected_project
            {
                projects[index].name = state.edit_project_name.to_string();
                projects[index].repo = state.edit_project_repo.to_string();
            }

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
        Err(err) => Err(err),
    }
}
