use std::{
    fs::{self, create_dir_all, read_to_string, remove_dir_all, remove_file, write},
    io::{self, ErrorKind, Write},
    path::PathBuf,
    str::FromStr,
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
    Import,
}

#[derive(Debug)]
pub struct ProjectCreationStatus
{
    pub creating: bool,
    pub failed: bool,
    pub log: Vec<String>,
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
    pub project_creation_status: ProjectCreationStatus,
    pub project_restoration_failed: bool,
    pub restoring_project: bool,
    pub import_project_path: String,
    pub import_project_name: String,
    pub import_project_name_changed: bool,
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
            project_creation_status: ProjectCreationStatus {
                creating: false,
                failed: false,
                log: vec![],
            },
            import_project_path: String::new(),
            import_project_name: String::new(),
            project_restoration_failed: false,
            delete_project_folder: false,
            new_project_path_changed: false,
            import_project_name_changed: false,
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

    pub fn get_config_dir(
        sub_dir: String,
        create_if_missing: Option<String>,
    ) -> Result<PathBuf, String>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            let config_path: PathBuf = proj_dirs.config_dir().to_path_buf();

            if let Err(err) = create_dir_all(&config_path)
            {
                return Err(format!("Error: Could not create config dir ({err})"));
            };

            let path: PathBuf = config_path.join(&sub_dir);

            if path.exists()
            {
                return Ok(path);
            }

            if let Some(content) = create_if_missing
                && let Err(err) = write(&path, content.as_bytes())
            {
                return Err(format!("Error: Could not write to {sub_dir} ({err})"));
            }
        }

        Err(String::from("Error: Could not find config dir"))
    }

    pub fn remove_project(&mut self) -> Result<(), String>
    {
        match AppState::get_config_dir(String::from("projects.json"), None)
        {
            Ok(projects_path) =>
            {
                if let Some(index) = self.selected_project
                {
                    if self.project_list[index].exists
                    {
                        if let Err(err) =
                            remove_file(self.project_list[index].path.join(".projman"))
                        {
                            return Err(format!("Error: Could not remove .projman file ({err})"));
                        }

                        if self.delete_project_folder
                            && let Err(err) = remove_dir_all(&self.project_list[index].path)
                        {
                            return Err(format!("Error: Could not remove project folder ({err})"));
                        }
                    }

                    let projects_from_json: String = match read_to_string(&projects_path)
                    {
                        Ok(json) => json,
                        Err(err) =>
                        {
                            return Err(format!("Error: Could not read projects.json ({err})"));
                        }
                    };

                    let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
                    {
                        Ok(projects) => projects,
                        Err(err) =>
                        {
                            return Err(format!("Error: Could not parse projects.json ({err})"));
                        }
                    };

                    projects.remove(index);

                    let projects_to_json: String = match serde_json::to_string_pretty(&projects)
                    {
                        Ok(string) => string,
                        Err(err) =>
                        {
                            return Err(format!("Error: Could not read projects.json ({err})"));
                        }
                    };

                    if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
                    {
                        return Err(format!("Error: Could not write to projects.json ({err})"));
                    };

                    self.project_list.remove(index);
                }

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn restore_project(
        selected_project: Option<usize>,
        project_list: Vec<Project>,
    ) -> Result<usize, String>
    {
        match AppState::get_config_dir(String::from("projects.json"), None)
        {
            Ok(projects_path) =>
            {
                if let Some(index) = selected_project
                {
                    let project = &project_list[index];

                    if !project.path.exists()
                        && let Err(err) = project.clone_repo()
                    {
                        return Err(format!("Error: Could not clone project repo ({err})"));
                    }

                    if !project.path.join(".projman").exists()
                    {
                        let mut projman_file: fs::File =
                            match fs::File::create_new(project.path.join(".projman"))
                            {
                                Ok(file) => file,
                                Err(err) =>
                                {
                                    return Err(format!(
                                        "Error: Could not create .projman file ({err})"
                                    ));
                                }
                            };

                        if let Err(err) =
                            projman_file.write_all(project.project_type.to_string().as_bytes())
                        {
                            return Err(format!("Error: Could not write to .projman file ({err})"));
                        }
                    }

                    let projects_from_json: String = match read_to_string(&projects_path)
                    {
                        Ok(json) => json,
                        Err(err) =>
                        {
                            return Err(format!("Error: Could not read projects.json ({err})"));
                        }
                    };

                    let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
                    {
                        Ok(projects) => projects,
                        Err(err) =>
                        {
                            return Err(format!("Error: Could not parse projects.json ({err})"));
                        }
                    };

                    projects[index].exists = true;

                    let projects_to_json: String = match serde_json::to_string_pretty(&projects)
                    {
                        Ok(json) => json,
                        Err(err) =>
                        {
                            return Err(format!("Error: Could not parse projects.json ({err})"));
                        }
                    };

                    if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
                    {
                        return Err(format!("Error: Could not write to projects.json ({err})"));
                    };

                    return Ok(index);
                }

                Err(String::from("Error: No prject selected"))
            }
            Err(err) => Err(err),
        }
    }

    pub fn import_project(&mut self) -> Result<(), String>
    {
        match AppState::get_config_dir(String::from("projects.json"), None)
        {
            Ok(projects_path) =>
            {
                let path: PathBuf = PathBuf::from(&self.config.general.projects_dir)
                    .join(&self.import_project_path);

                let name: String = self.import_project_name.to_string();

                let project_type: ProjectType = match &read_to_string(path.join(".projman"))
                {
                    Ok(string) => ProjectType::from_str(string)?,
                    Err(err) => return Err(format!("Error: Could not read .projman file ({err})")),
                };

                let repo: String = match Project::get_remote(&path)
                {
                    Ok(url) => url,
                    Err(err) => return Err(format!("Error: Could not get remote origin ({err})")),
                };

                let project: Project = Project {
                    exists: true,
                    name,
                    path,
                    project_type,
                    repo,
                };

                let projects_from_json: String = match read_to_string(&projects_path)
                {
                    Ok(json) => json,
                    Err(err) => return Err(format!("Error: Could not read projects.json ({err})")),
                };

                let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
                {
                    Ok(projects) => projects,
                    Err(err) =>
                    {
                        return Err(format!("Error: Could not parse projects.json ({err})"));
                    }
                };

                projects.push(project);

                let projects_to_json: String = match serde_json::to_string_pretty(&projects)
                {
                    Ok(json) => json,
                    Err(err) =>
                    {
                        return Err(format!("Error: Could not parse projects.json ({err})"));
                    }
                };

                if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
                {
                    return Err(format!("Error: Could not write to projects.json ({err})"));
                };

                Ok(())
            }
            Err(err) => Err(err),
        }
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
