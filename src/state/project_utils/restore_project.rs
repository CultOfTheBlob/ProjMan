use std::{
    fs::{self, read_to_string, write},
    io::Write,
};

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
};

pub async fn restore_project(index: usize, project_list: Vec<Project>) -> Result<usize, Error>
{
    match AppState::get_config_dir(String::from("projects.json"), None)
    {
        Ok(projects_path) =>
        {
            let project = &project_list[index];

            if !project.path.exists()
                && let Err(err) = project.clone_repo()
            {
                return Err(error!(Error::Clone, "project repo", err));
            }

            if !project.path.join(".projman").exists()
            {
                let mut projman_file: fs::File =
                    match fs::File::create_new(project.path.join(".projman"))
                    {
                        Ok(file) => file,
                        Err(err) =>
                        {
                            return Err(error!(Error::Create, ".projman file", err));
                        }
                    };

                if let Err(err) = projman_file.write_all(project.template.to_string().as_bytes())
                {
                    return Err(error!(Error::Write, ".projman file", err));
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

            projects[index].exists = true;

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

            Ok(index)
        }
        Err(err) => Err(err),
    }
}
