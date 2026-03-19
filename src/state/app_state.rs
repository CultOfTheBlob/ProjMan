use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

use color_eyre::owo_colors::OwoColorize;
use directories::ProjectDirs;
use iced::widget::combo_box;

use crate::{
    error::{Error, ErrorInfo},
    project::{Project, project_type::ProjectType},
    state::{
        config::Config,
        project_utils::{
            import_project::import_project, remove_project::remove_project,
            restore_project::restore_project,
        },
    },
};

#[derive(Debug)]
pub enum Popup
{
    Remove,
    Create,
    Import,
}

#[derive(Debug)]
pub struct ProjectCreationStatus
{
    pub creating: bool,
    pub failed: bool,
    pub log: Vec<String>,
}

#[derive(Debug)]
pub struct AppState
{
    pub config: Config,
    pub project_list: Vec<Project>,
    pub new_project: Project,
    pub new_project_path_changed: bool,
    pub project_types: combo_box::State<ProjectType>,
    pub selected_project: Option<usize>,
    pub delete_project_folder: bool,
    pub pending: Option<Popup>,
    pub project_creation_status: ProjectCreationStatus,
    pub project_restoration_failed: bool,
    pub restoring_project: bool,
    pub import_project_path: String,
    pub import_project_name: String,
    pub import_project_name_changed: bool,
}

impl Default for AppState
{
    fn default() -> Self
    {
        let project_list: Vec<Project> = match AppState::create_project_list_from_json()
        {
            Ok(projects) => projects,
            Err(err) => panic!("{}", err.get_message().red()),
        };

        Self {
            config: Config::default(),
            project_list,
            new_project: Project::default(&Config::default()),
            project_types: combo_box::State::new(ProjectType::ALL.to_vec()),
            project_creation_status: ProjectCreationStatus {
                creating: false,
                failed: false,
                log: vec![],
            },
            import_project_path: String::new(),
            import_project_name: String::new(),
            project_restoration_failed: false,
            delete_project_folder: false,
            new_project_path_changed: false,
            import_project_name_changed: false,
            restoring_project: false,
            selected_project: None,
            pending: None,
        }
    }
}

impl AppState
{
    pub fn with_config(self, config: Config) -> Self
    {
        AppState { config, ..self }
    }

    pub fn apply_config(self) -> Self
    {
        Self {
            new_project: Project::default(&self.config),
            delete_project_folder: self.config.general.delete_project_folder,
            ..self
        }
    }

    pub fn get_config_dir(
        sub_dir: String,
        create_if_missing: Option<String>,
    ) -> Result<PathBuf, Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            let config_path: PathBuf = proj_dirs.config_dir().to_path_buf();

            if let Err(err) = create_dir_all(&config_path)
            {
                return Err(Error::Create(ErrorInfo {
                    string: String::from("config dir"),
                    err: err.to_string(),
                }));
            };

            let path: PathBuf = config_path.join(&sub_dir);

            if path.exists()
            {
                return Ok(path);
            }

            if let Some(content) = create_if_missing
                && let Err(err) = write(&path, content.as_bytes())
            {
                return Err(Error::WriteTo(ErrorInfo {
                    string: sub_dir,
                    err: err.to_string(),
                }));
            }
        }

        Err(Error::Find(ErrorInfo {
            string: String::from("config dir"),
            err: String::new(),
        }))
    }

    pub fn remove_project(&mut self) -> Result<(), Error>
    {
        remove_project(self)
    }

    pub async fn restore_project(
        selected_project: Option<usize>,
        project_list: Vec<Project>,
    ) -> Result<usize, Error>
    {
        restore_project(selected_project, project_list).await
    }

    pub fn import_project(&mut self) -> Result<(), Error>
    {
        import_project(self)
    }

    pub fn create_project_list_from_json() -> Result<Vec<Project>, Error>
    {
        match AppState::get_config_dir(String::from("projects.json"), Some(String::from("[]")))
        {
            Ok(projects_path) =>
            {
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

                for project in &mut projects
                {
                    project.exists =
                        project.path.is_dir() && project.path.join(".projman").is_file();
                }

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
                    return Err(Error::WriteTo(ErrorInfo {
                        string: String::from("projects.json"),
                        err: err.to_string(),
                    }));
                };

                Ok(projects)
            }
            Err(err) => Err(err),
        }
    }
}
