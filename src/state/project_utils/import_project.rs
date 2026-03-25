use std::{
    fs::{read_to_string, write},
    path::PathBuf,
    str::FromStr,
};

use askalono::{Store, TextData};

use crate::{
    error::{Error, ErrorInfo},
    project::{Project, project_type::ProjectType},
    state::app_state::AppState,
};

pub fn import_project(state: &mut AppState) -> Result<(), Error>
{
    match AppState::get_config_dir(String::from("projects.json"), None)
    {
        Ok(projects_path) =>
        {
            let path: PathBuf =
                PathBuf::from(&state.config.general.projects_dir).join(&state.import_project_path);

            let name: String = state.import_project_name.to_string();

            let project_type: ProjectType = match &read_to_string(path.join(".projman"))
            {
                Ok(string) => ProjectType::from_str(string)?,
                Err(err) =>
                {
                    return Err(Error::Read(ErrorInfo {
                        string: String::from(".projman file"),
                        err: err.to_string(),
                    }));
                }
            };

            let repo: String = match Project::get_remote(&path)
            {
                Ok(url) => url,
                Err(err) =>
                {
                    return Err(Error::Fetch(ErrorInfo {
                        string: String::from("remote origin"),
                        err: err.to_string(),
                    }));
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
                        return Err(Error::Fetch(ErrorInfo {
                            string: String::from("project license"),
                            err: err.to_string(),
                        }));
                    }
                };

                let license_path: PathBuf = path.join("LICENSE");
                let license_contents: String = match read_to_string(&license_path)
                {
                    Ok(contents) => contents,
                    Err(err) =>
                    {
                        return Err(Error::Read(ErrorInfo {
                            string: String::from("LICENSE file"),
                            err: err.to_string(),
                        }));
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
                project_type,
                repo,
                license,
            };

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

            projects.push(project);

            let projects_to_json: String = match serde_json::to_string_pretty(&projects)
            {
                Ok(json) => json,
                Err(err) =>
                {
                    return Err(Error::Parse(ErrorInfo {
                        string: String::from("projects.json"),
                        err: err.to_string(),
                    }));
                }
            };

            if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
            {
                return Err(Error::Write(ErrorInfo {
                    string: String::from("projects.json"),
                    err: err.to_string(),
                }));
            };

            Ok(())
        }
        Err(err) => Err(err),
    }
}
