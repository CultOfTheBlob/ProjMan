use std::{io, path::PathBuf, process};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum CurrentScreen
{
    Main,
    Create,
}

#[derive(Debug)]
pub struct App
{
    pub current_screen: CurrentScreen,
    pub project_list: Vec<Project>,
    current_project_index: usize,
    pub creation_menu_tabs: Vec<String>,
    pub current_tab: Option<usize>,
}

impl App
{
    pub fn new() -> App
    {
        App {
            current_screen: CurrentScreen::Main,
            project_list: Vec::new(),
            current_project_index: 0,
            creation_menu_tabs: vec![
                String::from("Name"),
                String::from("Git Repo"),
                String::from("Path"),
                String::from("Type"),
            ],
            current_tab: Some(0),
        }
    }

    pub fn projects(self, mut project_list: Vec<Project>) -> App
    {
        project_list.retain(|project| -> bool {
            let path: PathBuf = PathBuf::from(&project.path);
            path.exists() && path.is_dir() && path.join(".projman").is_file()
        });

        App {
            project_list,
            ..self
        }
    }

    pub fn get_current_project_index(&self) -> &usize
    {
        &self.current_project_index
    }

    pub fn increment_current_project(&mut self)
    {
        if self.project_list.is_empty()
        {
            return;
        }

        let project_list_len: usize = self.project_list.len();

        if self.current_project_index >= project_list_len - 1
        {
            self.current_project_index = 0;
        }
        else
        {
            self.current_project_index += 1;
        }
    }

    pub fn decrement_current_project(&mut self)
    {
        if self.project_list.is_empty()
        {
            return;
        }

        let project_list_len: usize = self.project_list.len();

        if self.current_project_index == 0
        {
            self.current_project_index = project_list_len - 1;
        }
        else
        {
            self.current_project_index -= 1;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
