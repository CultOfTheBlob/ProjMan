use crate::{
    error::{Error, ErrorInfo},
    project::{Project, ProjmanFile},
    state::app_state::AppState,
};
use std::fs;

impl AppState {
    pub fn edit_project(&self) -> Result<(), Error> {
        if self.selected_project.is_none() {
            return Ok(());
        }

        let projects_path = Self::get_config_dir("projects.json", None)?;

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

        if let Some(index) = self.selected_project {
            let project = &mut projects[index];

            project.name = self.edit_project_name.clone();
            project.repo = self.edit_project_repo.clone();

            let projman_path = project.path.join(ProjmanFile::FILE_NAME);

            let projman_file_from_toml = match &fs::read_to_string(&projman_path) {
                Ok(string) => string.clone(),
                Err(err) => {
                    return Err(error!(Error::Read, ProjmanFile::FILE_NAME, err));
                }
            };

            let mut projman_file: ProjmanFile = match toml::from_str(&projman_file_from_toml) {
                Ok(projman_file) => projman_file,
                Err(err) => return Err(error!(Error::Parse, ProjmanFile::FILE_NAME, err)),
            };

            projman_file.name = self.edit_project_name.clone();
            projman_file.repo = self.edit_project_repo.clone();

            let projman_file_to_toml = match toml::to_string_pretty(&projman_file) {
                Ok(json) => json,
                Err(err) => {
                    return Err(error!(Error::Parse, ProjmanFile::FILE_NAME, err));
                }
            };

            if let Err(err) = fs::write(&projman_path, projman_file_to_toml.as_bytes()) {
                return Err(error!(Error::Write, ProjmanFile::FILE_NAME, err));
            }
        }

        let projects_to_json = match serde_json::to_string_pretty(&projects) {
            Ok(json) => json,
            Err(err) => {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        if let Err(err) = fs::write(&projects_path, projects_to_json.as_bytes()) {
            return Err(error!(Error::Write, "projects.json", err));
        }

        Ok(())
    }
}
