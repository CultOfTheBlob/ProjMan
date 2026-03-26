use std::{
    fs::{read_to_string, write},
    path::PathBuf,
    str::FromStr,
};

use askalono::{Store, TextData};

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
    templates::template::Template,
};

impl AppState
{
    pub fn import_project(&mut self) -> Result<(), Error>
    {
        let projects_path: PathBuf = AppState::get_config_dir(String::from("projects.json"), None)?;

        let path: PathBuf =
            PathBuf::from(&self.config.general.projects_dir).join(&self.import_project_path);

        let name: String = self.import_project_name.to_string();

        let project_type: Template = match &read_to_string(path.join(".projman"))
        {
            Ok(string) => Template::from_str(string)?,
            Err(err) =>
            {
                return Err(error!(Error::Read, ".projman file", err));
            }
        };

        let repo: String = match Project::get_remote(&path)
        {
            Ok(url) => url,
            Err(err) =>
            {
                return Err(error!(Error::Fetch, "remote origin", err));
            }
        };

        let license: String = {
            let store: Store = match Store::from_cache(
                &include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/cache/license.cache.zstd"
                ))[..],
            )
            {
                Ok(store) => store,
                Err(err) =>
                {
                    return Err(error!(Error::Fetch, "project license", err));
                }
            };

            let license_path: PathBuf = path.join("LICENSE");
            let license_contents: String = match read_to_string(&license_path)
            {
                Ok(contents) => contents,
                Err(err) =>
                {
                    return Err(error!(Error::Read, "LICENSE file", err));
                }
            };

            store
                .analyze(&TextData::from(license_contents.as_str()))
                .name
                .to_string()
        };

        let project: Project = Project {
            exists: true,
            name,
            path,
            template: project_type,
            repo,
            license,
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
