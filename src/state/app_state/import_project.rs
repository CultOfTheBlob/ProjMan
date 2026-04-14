use crate::{
    error::{Error, ErrorInfo},
    project::{Project, ProjmanFile},
    state::app_state::AppState,
};
use std::{fs, path::PathBuf};

impl AppState
{
    pub fn import_project(&self) -> Result<(), Error>
    {
        let projects_path = Self::get_config_dir("projects.json", None)?;

        let path = PathBuf::from(&self.config.general.projects_dir).join(&self.import_project_path);

        let project_from_toml = match &fs::read_to_string(path.join(ProjmanFile::FILE_NAME))
        {
            Ok(string) => string.clone(),
            Err(err) =>
            {
                return Err(error!(Error::Read, ProjmanFile::FILE_NAME, err));
            }
        };

        let projman_file: ProjmanFile = match toml::from_str(&project_from_toml)
        {
            Ok(projman_file) => projman_file,
            Err(err) => return Err(error!(Error::Parse, ProjmanFile::FILE_NAME, err)),
        };

        let project = Project {
            exists: true,
            name: projman_file.name,
            path,
            template: self.templates.get(&projman_file.template_name)?,
            template_name: projman_file.template_name,
            repo: projman_file.repo,
            license: projman_file.license,
        };

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

        projects.push(project);

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
