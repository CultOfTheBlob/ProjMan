use std::{
    fs::{File, create_dir_all, read_to_string, remove_dir_all, remove_file, write},
    io::{self},
    mem::replace,
    path::PathBuf,
    process,
};

use color_eyre::owo_colors::OwoColorize;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug)]
pub enum Popup
{
    Remove,
}

#[derive(Debug)]
pub struct AppState
{
    pub config: Config,
    pub project_list: Vec<Project>,
    pub new_project: Project,
    pub selected_project: Option<usize>,
    pub delete_project_folder: bool,
    pub pending: Option<Popup>,
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
            delete_project_folder: false,
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

    pub fn create_project(&mut self) -> Result<(), std::io::Error>
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

            create_dir_all(&self.new_project.path)?;
            File::create_new(self.new_project.path.join(".projman"))?;

            let mut projects_json: Vec<Project> =
                serde_json::from_str(&read_to_string(&data_path)?)?;

            projects_json.push(replace(
                &mut self.new_project,
                Project::default(&self.config),
            ));

            write(
                &data_path,
                serde_json::to_string_pretty(&projects_json)?.as_bytes(),
            )?;

            self.project_list = projects_json;

            self.new_project = Project::default(&self.config);

            return Ok(());
        }

        Ok(())
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ProjectType
{
    #[default]
    Test,
}

impl ProjectType
{
    fn run(&self, project: &Project) -> io::Result<()>
    {
        match self
        {
            ProjectType::Test =>
            {
                process::Command::new("kitty")
                    .arg("--detach")
                    .current_dir(&project.path)
                    .spawn()?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project
{
    pub name: String,
    pub path: PathBuf,
    pub project_type: ProjectType,
}

impl Project
{
    pub fn run(&self) -> io::Result<()>
    {
        self.project_type.run(self)
    }

    pub fn default(config: &Config) -> Self
    {
        let name: &str = "NewProject";

        Self {
            name: String::from(name),
            path: PathBuf::from(&config.general.projects_dir).join(name),
            project_type: ProjectType::default(),
        }
    }
}
