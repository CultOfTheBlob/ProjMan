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
    current_project_index: usize,
}

impl App
{
    pub fn new() -> App
    {
        App {
            current_screen: CurrentScreen::Main,
            project_list: Vec::new(),
            current_project_index: 0,
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
        let project_list_len: usize = self.project_list.len();

        if self.current_project_index >= project_list_len - 1
        {
            self.current_project_index = 0;
        }
        else
        {
            self.current_project_index += 1
        }
    }

    pub fn decrement_current_project(&mut self)
    {
        let project_list_len: usize = self.project_list.len();

        if self.current_project_index == 0
        {
            self.current_project_index = project_list_len - 1;
        }
        else
        {
            self.current_project_index -= 1
        }
    }
}
