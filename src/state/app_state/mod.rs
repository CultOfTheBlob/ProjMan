use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
    sync::Arc,
};

use directories::ProjectDirs;
use iced::widget::combo_box;

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::config::Config,
    templates::{Templates, template::Template},
};

mod edit_project;
mod import_project;
mod load_projects;
mod remove_project;
mod restore_project;
mod update_project;

#[derive(Debug)]
pub struct AppState
{
    pub config: Config,
    pub project_list: Arc<Vec<Project>>,
    pub new_project: Arc<Project>,
    pub new_project_path_changed: bool,
    pub project_templates: combo_box::State<Template>,
    pub selected_project: Option<usize>,
    pub delete_project_folder: bool,
    pub pending: Option<Popup>,
    pub project_creation_status: ProjectCreationStatus,
    pub project_restoration_failed: bool,
    pub restoring_project: bool,
    pub import_project_path: String,
    pub edit_project_name: String,
    pub edit_project_repo: String,
    pub notifications: Vec<Notification>,
    pub sidebar_expanded: bool,
}

impl Default for AppState
{
    fn default() -> Self
    {
        let mut notifications = vec![];

        let project_list: Arc<Vec<Project>> = match AppState::load_projects()
        {
            Ok(projects) => Arc::new(projects),
            Err(err) =>
            {
                notifications.push(Notification {
                    text: err.get_message(),
                    kind: NotifKind::Error,
                });

                Arc::new(vec![])
            }
        };

        Self {
            config: Config::default(),
            project_list,
            notifications,
            new_project: Arc::new(Project::default(&Config::default())),
            project_templates: combo_box::State::new(Templates::default().templates().to_vec()),
            project_creation_status: ProjectCreationStatus {
                creating: false,
                failed: false,
                step: 0.0,
                log: vec![],
            },
            import_project_path: String::new(),
            edit_project_name: String::new(),
            edit_project_repo: String::new(),
            project_restoration_failed: false,
            delete_project_folder: false,
            new_project_path_changed: false,
            restoring_project: false,
            sidebar_expanded: true,
            selected_project: None,
            pending: None,
        }
    }
}

impl AppState
{
    pub fn templates(self, templates: Vec<Template>) -> Self
    {
        Self {
            project_templates: combo_box::State::new(templates),
            ..self
        }
    }

    pub fn config(self, config: Config) -> Self
    {
        Self {
            new_project: Arc::new(Project::default(&config)),
            delete_project_folder: config.general.delete_project_folder,
            config,
            ..self
        }
    }

    pub fn push_notification(&mut self, text: String, kind: NotifKind)
    {
        for notification in &self.notifications
        {
            if notification.text == text
            {
                return;
            }
        }

        self.notifications.push(Notification { text, kind });
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
                return Err(error!(Error::Create, "config dir", err));
            };

            let path: PathBuf = config_path.join(&sub_dir);

            if path.exists()
            {
                return Ok(path);
            }

            if let Some(content) = create_if_missing
                && let Err(err) = write(&path, content.as_bytes())
            {
                return Err(error!(Error::Write, sub_dir, err));
            }

            return Ok(path);
        }

        Err(error!(Error::Find, "config dir", ""))
    }
}

#[derive(Debug)]
pub enum Popup
{
    Remove,
    Create,
    Import,
    Edit,
}

#[derive(Debug)]
pub struct ProjectCreationStatus
{
    pub creating: bool,
    pub failed: bool,
    pub step: f32,
    pub log: Vec<String>,
}

#[derive(Debug)]
pub enum NotifKind
{
    Warning,
    Error,
}

#[derive(Debug)]
pub struct Notification
{
    pub text: String,
    pub kind: NotifKind,
}
