use std::{
    fs::{
        self, create_dir, create_dir_all, read_to_string, remove_dir, remove_dir_all, remove_file,
        write,
    },
    io::{self, ErrorKind, Write},
    path::PathBuf,
    process::{self},
};

use color_eyre::owo_colors::OwoColorize;
use directories::ProjectDirs;
use iced::{Task, widget::combo_box};

use crate::{
    message::Message,
    state::{config::Config, project::Project, project_type::ProjectType},
    templates::{Command, File, Folder, TemplateConfig},
};

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
    pub project_restoration_failed: bool,
    pub restoring_project: bool,
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
            project_restoration_failed: false,
            delete_project_folder: false,
            new_project_path_changed: false,
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

    pub fn remove_project(&mut self) -> Result<(), std::io::Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            create_dir_all(proj_dirs.config_dir())?;

            let config_path: PathBuf = proj_dirs.config_dir().join("projects.json");

            if !config_path.is_file()
            {
                return Err(std::io::Error::new(
                    io::ErrorKind::NotADirectory,
                    "Error: projects.json does not exist",
                ));
            }

            if let Some(index) = self.selected_project
            {
                if self.project_list[index].exists
                {
                    remove_file(self.project_list[index].path.join(".projman"))?;

                    if self.delete_project_folder
                    {
                        remove_dir_all(&self.project_list[index].path)?;
                    }
                }

                let mut projects_json: Vec<Project> =
                    serde_json::from_str(&read_to_string(&config_path)?)?;

                projects_json.remove(index);

                write(
                    &config_path,
                    serde_json::to_string_pretty(&projects_json)?.as_bytes(),
                )?;

                self.project_list.remove(index);
            }

            return Ok(());
        }

        Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Could not find config folder",
        ))
    }

    pub async fn restore_project(
        selected_project: Option<usize>,
        project_list: Vec<Project>,
    ) -> Result<usize, std::io::Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            create_dir_all(proj_dirs.config_dir())?;

            let config_dir: PathBuf = proj_dirs.config_dir().join("projects.json");

            if !config_dir.is_file()
            {
                return Err(std::io::Error::new(
                    io::ErrorKind::NotADirectory,
                    "Error: projects.json does not exist",
                ));
            }

            if let Some(index) = selected_project
            {
                let project = &project_list[index];

                if !project.path.exists()
                {
                    project.clone_repo()?;
                }

                if !project.path.join(".projman").exists()
                {
                    fs::File::create_new(project.path.join(".projman"))?;
                }

                let mut projects_json: Vec<Project> =
                    serde_json::from_str(&read_to_string(&config_dir)?)?;

                projects_json[index].exists = true;

                write(
                    &config_dir,
                    serde_json::to_string_pretty(&projects_json)?.as_bytes(),
                )?;

                return Ok(index);
            }
        }

        Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Could not find config folder",
        ))
    }

    // fn create_config_dir() -> Result<PathBuf, String>
    // {
    //     if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
    //     {
    //         let config_path: PathBuf = proj_dirs.config_dir().join("projects.json");
    //
    //         if !config_path.is_file()
    //         {
    //             return Err("Error: File projects.json does not exist".to_string());
    //         }
    //
    //         return Ok(config_path);
    //     }
    //
    //     Err("Could not find config dir".to_string())
    // }

    pub fn create_project(project: Project) -> Task<Message>
    {
        let project_template: TemplateConfig = match project.project_type.template()
        {
            Ok(template) => template,
            Err(err) =>
            {
                return Task::done(Message::ProgressCreate(format!(
                    "Error: Could not get project template ({err})"
                )));
            }
        };

        Task::perform(
            AppState::create_project_dir(project.path.clone()),
            Message::ProgressCreate,
        )
        .chain(Task::perform(
            AppState::clone_project_repo(project.clone()),
            Message::ProgressCreate,
        ))
        .chain(Task::perform(
            AppState::create_projman_file(project.path.clone()),
            Message::ProgressCreate,
        ))
        .chain(Task::perform(
            AppState::create_dir_structure(
                project_template.dir_structure.clone(),
                project.path.clone(),
            ),
            Message::ProgressCreate,
        ))
        .chain(Task::perform(
            AppState::create_project_files(project_template.files.clone(), project.path.clone()),
            Message::ProgressCreate,
        ))
        .chain(Task::perform(
            AppState::execute_build_command(
                project_template.build.first().unwrap().clone(),
                project.path.clone(),
            ),
            |status: bool| Message::FinishBuildCommand(0, status),
        ))
    }

    async fn create_project_dir(project_path: PathBuf) -> String
    {
        match create_dir_all(&project_path)
        {
            Ok(_) => "Created project dir...".to_string(),
            Err(err) => format!("Error: Could not create project directory ({err})"),
        }
    }

    async fn clone_project_repo(project: Project) -> String
    {
        match project.clone_repo()
        {
            Ok(_) => "Cloned project repo...".to_string(),

            Err(err) =>
            {
                let _ = remove_dir(&project.path);
                format!("Error: Could not clone project repo ({err})")
            }
        }
    }

    async fn create_projman_file(project_path: PathBuf) -> String
    {
        match fs::File::create_new(project_path.join(".projman"))
        {
            Ok(_) => "Created .projman file...".to_string(),
            Err(err) => format!("Error: Could not create .projman file {err}"),
        }
    }

    async fn create_dir_structure(
        project_dir_structure: Vec<Folder>,
        project_path: PathBuf,
    ) -> String
    {
        for dir in &project_dir_structure
        {
            let dirs: Vec<PathBuf> = dir.parse(&project_path);

            for dir in &dirs
            {
                if let Err(err) = create_dir(dir)
                {
                    return format!("Error: Could not create directory structure ({err})");
                }
            }
        }

        "Created project directory structure...".to_string()
    }

    async fn create_project_files(project_files: Vec<File>, project_path: PathBuf) -> String
    {
        for file in &project_files
        {
            if let Err(err) = write(project_path.join(&file.path), &file.content)
            {
                return format!("Error: Could not create project files ({err})");
            };
        }

        "Created project files...".to_string()
    }

    pub async fn execute_build_command(command: Command, project_path: PathBuf) -> bool
    {
        process::Command::new(&command.program)
            .args(&command.args)
            .current_dir(&project_path)
            .status()
            .is_ok()
    }

    pub async fn add_project_to_json(project: Project) -> Result<Vec<Project>, String>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            if let Err(err) = create_dir_all(proj_dirs.config_dir())
            {
                return Err(format!("Error: Could not create config dir ({err})"));
            };

            let config_path: PathBuf = proj_dirs.config_dir().join("projects.json");

            if !config_path.is_file()
            {
                return Err("Error: projects.json does not exist".to_string());
            }

            let projects_from_json: String = match read_to_string(&config_path)
            {
                Ok(string) => string,
                Err(err) => return Err(format!("Error: Could not read projects.json ({err})")),
            };

            let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
            {
                Ok(it) => it,
                Err(err) => return Err(format!("Error: Could not parse projects.json ({err})")),
            };

            projects.push(project);

            let projects_to_json = match serde_json::to_string_pretty(&projects)
            {
                Ok(it) => it,
                Err(err) => return Err(format!("Error: Could not parse projects.json ({err})")),
            };

            if let Err(err) = write(&config_path, projects_to_json.as_bytes())
            {
                return Err(format!("Error: Could not write to projects.json ({err})"));
            };

            return Ok(projects);
        }

        Err("Error: Could not find config dir".to_string())
    }

    pub fn create_project_list_from_json() -> Result<Vec<Project>, std::io::Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            create_dir_all(proj_dirs.config_dir())?;

            let config_path: PathBuf = proj_dirs.config_dir().join("projects.json");

            if !config_path.is_file()
            {
                fs::File::create(&config_path)?.write_all("[]".as_bytes())?;

                return Ok(Vec::<Project>::new());
            }

            let mut projects_json: Vec<Project> =
                serde_json::from_str(&read_to_string(&config_path)?)?;

            for project in &mut projects_json
            {
                project.exists = project.path.is_dir() && project.path.join(".projman").is_file();
            }

            write(
                &config_path,
                serde_json::to_string_pretty(&projects_json)?.as_bytes(),
            )?;

            return Ok(projects_json);
        }

        Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Could not find config folder",
        ))
    }
}
