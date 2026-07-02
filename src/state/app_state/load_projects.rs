use crate::{
    error::{Error, ErrorInfo},
    project::{Project, ProjmanFile},
    state::app_state::AppState,
};
use std::fs;

impl AppState {
    pub fn load_projects(&self) -> Result<Vec<Project>, Error> {
        let projects_path = Self::get_config_dir("projects.json", Some(String::from("[]")))?;

        let projects_from_json = match fs::read_to_string(&projects_path) {
            Ok(json) => json,
            Err(err) => {
                return Err(error!(Error::Read, "projects.json", err));
            }
        };

        let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json) {
            Ok(projects) => projects,
            Err(err) => {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        for project in &mut projects {
            project.template = self.templates.get(&project.template_name)?;

            if !project.path.is_dir() {
                project.exists = false;
                continue;
            }

            if !project.path.join(ProjmanFile::FILE_NAME).is_file() {
                project.exists = false;
                continue;
            }

            let projman_file_is_correct: bool = {
                let project_from_toml =
                    match &fs::read_to_string(project.path.join(ProjmanFile::FILE_NAME)) {
                        Ok(string) => string.clone(),
                        Err(err) => {
                            return Err(error!(Error::Read, ProjmanFile::FILE_NAME, err));
                        }
                    };

                let projman_file: ProjmanFile = match toml::from_str(&project_from_toml) {
                    Ok(projman_file) => projman_file,
                    Err(err) => return Err(error!(Error::Parse, ProjmanFile::FILE_NAME, err)),
                };

                projman_file.name == project.name
                    && projman_file.template_name == project.template_name
                    && projman_file.repo == project.repo
                    && projman_file.license == project.license
            };

            project.exists = projman_file_is_correct;
        }

        let projects_to_json = match serde_json::to_string_pretty(&projects) {
            Ok(json) => json,
            Err(err) => {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        if projects_to_json == projects_from_json {
            return Ok(projects);
        }

        if let Err(err) = fs::write(&projects_path, projects_to_json.as_bytes()) {
            return Err(error!(Error::Write, "projects.json", err));
        }

        Ok(projects)
    }
}
