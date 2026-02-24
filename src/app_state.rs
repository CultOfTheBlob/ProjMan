use std::{
    fs::{File, create_dir_all, read_to_string, remove_file, write},
    io::{self},
    path::PathBuf,
    process,
};

use color_eyre::owo_colors::OwoColorize;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AppState
{
    pub project_list: Vec<Project>,
    pub selected_project: Option<usize>,
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
            project_list,
            selected_project: None,
        }
    }
}

impl AppState
{
    pub fn remove_project(&mut self, index: usize) -> Result<(), std::io::Error>
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

            remove_file(self.project_list[index].path.join(".projman"))?;

            let mut projects_json: Vec<Project> =
                serde_json::from_str(&read_to_string(&data_path)?)?;

            projects_json.remove(index);

            write(
                &data_path,
                serde_json::to_string_pretty(&projects_json)?.as_bytes(),
            )?;

            self.project_list.remove(index);

            return Ok(());
        }

        Ok(())
    }

    pub fn add_project(&mut self, project: Project) -> Result<(), std::io::Error>
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

            create_dir_all(&project.path)?;
            File::create_new(project.path.join(".projman"))?;

            let mut projects_json: Vec<Project> =
                serde_json::from_str(&read_to_string(&data_path)?)?;

            projects_json.push(project);

            write(
                &data_path,
                serde_json::to_string_pretty(&projects_json)?.as_bytes(),
            )?;

            self.project_list = projects_json;

            return Ok(());
        }

        Ok(())
    }

    pub fn create_project_list_from_json() -> Result<Vec<Project>, std::io::Error>
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

            return Ok(projects_json);
        }

        Ok(Vec::<Project>::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType
{
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
}
