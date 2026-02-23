use std::{
    fs::{File, create_dir_all, read_to_string},
    io,
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
        let project_list: Vec<Project> = match AppState::create_project_list()
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
    fn create_project_list() -> Result<Vec<Project>, std::io::Error>
    {
        match ProjectDirs::from("", "", "projman")
        {
            Some(proj_dirs) =>
            {
                create_dir_all(proj_dirs.data_dir())?;

                let data_path: PathBuf = proj_dirs.data_dir().join("projects.json");

                if data_path.is_file()
                {
                    let mut proj: Vec<Project> =
                        serde_json::from_str(&read_to_string(&data_path)?)?;

                    proj.retain(|project| -> bool {
                        let path: PathBuf = PathBuf::from(&project.path);
                        path.exists() && path.is_dir() && path.join(".projman").is_file()
                    });

                    Ok(proj)
                }
                else
                {
                    File::create(&data_path)?;

                    Ok(Vec::<Project>::new())
                }
            }
            _ => Ok(Vec::<Project>::new()),
        }
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
