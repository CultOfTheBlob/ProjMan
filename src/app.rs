use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub enum CurrentScreen
{
    Main,
}

#[derive(Debug)]
pub enum ProjectType
{
    Test,
}

#[derive(Debug)]
pub struct Project
{
    pub path: PathBuf,
    pub project_type: ProjectType,
}

#[derive(Debug)]
pub struct App
{
    pub current_screen: CurrentScreen,
    pub project_list: HashMap<String, Project>,
}

impl App
{
    pub fn new() -> App
    {
        App {
            current_screen: CurrentScreen::Main,
            project_list: HashMap::new(),
        }
    }

    /**
    Returns App identical to self but with a filled out project_list field.

    The project_list field includes every item in the project_list input that
    has a valid directory containing a .projman file.
    */
    pub fn projects(self, mut project_list: HashMap<String, Project>) -> App
    {
        project_list.retain(|_, project| -> bool {
            let path: PathBuf = PathBuf::from(&project.path);
            path.exists() && path.is_dir() && path.join(".projman").is_file()
        });

        App {
            project_list,
            ..self
        }
    }
}
