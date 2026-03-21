use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
};
use std::fs::{read_to_string, remove_dir_all, remove_file, write};

pub fn remove_project(state: &mut AppState) -> Result<(), Error>
{
    if state.selected_project.is_none()
    {
        return Ok(());
    }

    match AppState::get_config_dir(String::from("projects.json"), None)
    {
        Ok(projects_path) =>
        {
            if let Some(index) = state.selected_project
                && state.project_list[index].exists
            {
                if let Err(err) = remove_file(state.project_list[index].path.join(".projman"))
                {
                    return Err(Error::Remove(ErrorInfo {
                        string: String::from(".projman file"),
                        err: err.to_string(),
                    }));
                }

                if state.delete_project_folder
                    && let Err(err) = remove_dir_all(&state.project_list[index].path)
                {
                    return Err(Error::Remove(ErrorInfo {
                        string: String::from("project folder"),
                        err: err.to_string(),
                    }));
                }
            }

            let projects_from_json: String = match read_to_string(&projects_path)
            {
                Ok(json) => json,
                Err(err) =>
                {
                    return Err(Error::Read(ErrorInfo {
                        string: String::from("projects.json"),
                        err: err.to_string(),
                    }));
                }
            };

            let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
            {
                Ok(projects) => projects,
                Err(err) =>
                {
                    return Err(Error::Parse(ErrorInfo {
                        string: String::from("projects.json"),
                        err: err.to_string(),
                    }));
                }
            };

            if let Some(index) = state.selected_project
            {
                projects.remove(index);
            }

            let projects_to_json: String = match serde_json::to_string_pretty(&projects)
            {
                Ok(string) => string,
                Err(err) =>
                {
                    return Err(Error::Read(ErrorInfo {
                        string: String::from("projects.json"),
                        err: err.to_string(),
                    }));
                }
            };

            if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
            {
                return Err(Error::WriteTo(ErrorInfo {
                    string: String::from("projects.json"),
                    err: err.to_string(),
                }));
            };

            if let Some(index) = state.selected_project
            {
                state.project_list.remove(index);
            }

            Ok(())
        }
        Err(err) => Err(err),
    }
}
