use crate::{
    error::{Error, ErrorInfo},
    project::{Project, ProjmanFile},
    state::app_state::AppState,
};
use iced::futures::future::{self, Ready};
use std::{
    fs::{self, File as FsFile},
    io::Write as _,
    sync::Arc,
};

impl AppState
{
    pub fn restore_project(
        selected_project: Option<usize>,
        project_list: &Arc<Vec<Project>>,
    ) -> Ready<Result<usize, Error>>
    {
        future::ready((|| {
            let projects_path = Self::get_config_dir("projects.json", None)?;
            let Some(index) = selected_project
            else
            {
                return Err(Error::Other(String::from("Error: No project selected")));
            };
            let project = &project_list[index];
            if !project.path.exists()
                && let Err(err) = project.clone_repo()
            {
                return Err(error!(Error::Clone, "project repo", err));
            }
            else
            {
                let mut projman_file = match FsFile::create(project.path.join("projman.toml"))
                {
                    Ok(file) => file,
                    Err(err) => return Err(error!(Error::Create, "projman.toml", err)),
                };
                let projman_content = ProjmanFile {
                    name: project.name.clone(),
                    template_name: project.template_name.clone(),
                    repo: project.repo.clone(),
                    license: project.license.clone(),
                };
                let project_to_toml = match toml::to_string_pretty(&projman_content)
                {
                    Ok(string) => string,
                    Err(err) => return Err(error!(Error::Parse, "project", err)),
                };
                if let Err(err) = projman_file.write_all(project_to_toml.as_bytes())
                {
                    return Err(error!(Error::Write, "projman.toml", err));
                }
            }
            let projects_from_json = match fs::read_to_string(&projects_path)
            {
                Ok(json) => json,
                Err(err) => return Err(error!(Error::Read, "projects.json", err)),
            };
            let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
            {
                Ok(projects) => projects,
                Err(err) => return Err(error!(Error::Parse, "projects.json", err)),
            };
            projects[index].exists = true;
            let projects_to_json = match serde_json::to_string_pretty(&projects)
            {
                Ok(json) => json,
                Err(err) => return Err(error!(Error::Parse, "projects.json", err)),
            };
            if let Err(err) = fs::write(&projects_path, projects_to_json.as_bytes())
            {
                return Err(error!(Error::Write, "projects.json", err));
            }
            Ok(index)
        })())
    }
}
