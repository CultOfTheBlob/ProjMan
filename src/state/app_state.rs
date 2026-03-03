use std::{
    fs::{File, create_dir_all, read_to_string, remove_dir, remove_dir_all, remove_file, write},
    io::{self, ErrorKind},
    path::PathBuf,
};

use color_eyre::owo_colors::OwoColorize;
use directories::ProjectDirs;
use iced::widget::combo_box;

use crate::state::{config::Config, project::Project, project_type::ProjectType};

#[derive(Debug)]
pub enum Popup
{
    Remove,
    Create,
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
    pub project_creation_status: (bool, String),
}

impl Default for AppState
{
    fn default() -> Self
    {
        let project_list: Vec<Project> = match AppState::create_project_list_from_json()
        {
            Ok(projects) => projects,
            Err(err) => panic!("{}", err.to_string().red()),
        };

        Self {
            config: Config::default(),
            project_list,
            new_project: Project::default(&Config::default()),
            project_types: combo_box::State::new(ProjectType::ALL.to_vec()),
            project_creation_status: (false, String::new()),
            delete_project_folder: false,
            new_project_path_changed: false,
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

    pub fn remove_project(&mut self) -> Result<(), std::io::Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            create_dir_all(proj_dirs.data_dir())?;

            let data_path: PathBuf = proj_dirs.data_dir().join("projects.json");

            if !data_path.is_file()
            {
                return Err(std::io::Error::new(
                    io::ErrorKind::NotADirectory,
                    "Error: projects.json does not exist",
                ));
            }

            if let Some(index) = self.selected_project
            {
                remove_file(self.project_list[index].path.join(".projman"))?;

                if self.delete_project_folder
                {
                    remove_dir_all(&self.project_list[index].path)?;
                }

                let mut projects_json: Vec<Project> =
                    serde_json::from_str(&read_to_string(&data_path)?)?;

                projects_json.remove(index);

                write(
                    &data_path,
                    serde_json::to_string_pretty(&projects_json)?.as_bytes(),
                )?;

                self.project_list.remove(index);
            }

            return Ok(());
        }

        Ok(())
    }

    pub async fn create_project(new_project: Project) -> Result<Vec<Project>, std::io::Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            create_dir_all(proj_dirs.data_dir())?;

            let data_path: PathBuf = proj_dirs.data_dir().join("projects.json");

            if !data_path.is_file()
            {
                return Err(std::io::Error::new(
                    io::ErrorKind::NotADirectory,
                    "Error: projects.json does not exist",
                ));
            }

            create_dir_all(&new_project.path)?;

            if let Err(err) = new_project.clone_repo()
            {
                remove_dir(&new_project.path)?;
                return Err(err);
            };

            File::create_new(new_project.path.join(".projman"))?;

            let mut projects_json: Vec<Project> =
                serde_json::from_str(&read_to_string(&data_path)?)?;

            projects_json.push(new_project);

            write(
                &data_path,
                serde_json::to_string_pretty(&projects_json)?.as_bytes(),
            )?;

            return Ok(projects_json);
        }

        Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Could not find data folder",
        ))
    }

    fn create_project_list_from_json() -> Result<Vec<Project>, std::io::Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            create_dir_all(proj_dirs.data_dir())?;

            let data_path: PathBuf = proj_dirs.data_dir().join("projects.json");

            if !data_path.is_file()
            {
                File::create(&data_path)?;

                return Ok(Vec::<Project>::new());
            }
            let mut projects_json: Vec<Project> =
                serde_json::from_str(&read_to_string(&data_path)?)?;

            projects_json.retain(|project| -> bool {
                let path: PathBuf = PathBuf::from(&project.path);
                path.exists() && path.is_dir() && path.join(".projman").is_file()
            });

            write(
                &data_path,
                serde_json::to_string_pretty(&projects_json)?.as_bytes(),
            )?;

            return Ok(projects_json);
        }

        Ok(Vec::<Project>::new())
    }
}
