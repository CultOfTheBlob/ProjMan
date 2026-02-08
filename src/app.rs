use std::path::PathBuf;

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
    pub name: String,
    pub path: PathBuf,
    pub project_type: ProjectType,
}

#[derive(Debug)]
pub struct App
{
    pub current_screen: CurrentScreen,
    pub project_list: Vec<Project>,
}

impl App
{
    pub fn new() -> App
    {
        App {
            current_screen: CurrentScreen::Main,
            project_list: Vec::new(),
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
}
