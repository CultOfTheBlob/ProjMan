use std::{
    fs::{self, create_dir_all, read_to_string, remove_dir_all, remove_file, write},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

use color_eyre::owo_colors::OwoColorize;
use directories::ProjectDirs;
use iced::widget::combo_box;

use crate::{
    error::{Error, ErrorInfo},
    state::{config::Config, project::Project, project_type::ProjectType},
};

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
            Err(err) => panic!("{}", err.get_message().red()),
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
    ) -> Result<PathBuf, Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            let config_path: PathBuf = proj_dirs.config_dir().to_path_buf();

            if let Err(err) = create_dir_all(&config_path)
            {
                return Err(Error::Create(ErrorInfo {
                    string: String::from("config dir"),
                    err: err.to_string(),
                }));
            };

            let path: PathBuf = config_path.join(&sub_dir);

            if path.exists()
            {
                return Ok(path);
            }

            if let Some(content) = create_if_missing
                && let Err(err) = write(&path, content.as_bytes())
            {
                return Err(Error::WriteTo(ErrorInfo {
                    string: sub_dir,
                    err: err.to_string(),
                }));
            }
        }

        Err(Error::Find(ErrorInfo {
            string: String::from("config dir"),
            err: String::new(),
        }))
    }

    pub fn remove_project(&mut self) -> Result<(), Error>
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
                            return Err(Error::Remove(ErrorInfo {
                                string: String::from(".projman file"),
                                err: err.to_string(),
                            }));
                        }

                        if self.delete_project_folder
                            && let Err(err) = remove_dir_all(&self.project_list[index].path)
                        {
                            return Err(Error::Remove(ErrorInfo {
                                string: String::from("project folder"),
                                err: err.to_string(),
                            }));
                        }
                    }

                    let projects_from_json: String = match read_to_string(&projects_path)
                    {
                        Ok(json) => json,
                        Err(err) =>
                        {
                            return Err(Error::Read(ErrorInfo {
                                string: String::from("projects.json"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
                    {
                        Ok(projects) => projects,
                        Err(err) =>
                        {
                            return Err(Error::Parse(ErrorInfo {
                                string: String::from("projects.json"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    projects.remove(index);

                    let projects_to_json: String = match serde_json::to_string_pretty(&projects)
                    {
                        Ok(string) => string,
                        Err(err) =>
                        {
                            return Err(Error::Read(ErrorInfo {
                                string: String::from("projects.json"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
                    {
                        return Err(Error::WriteTo(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
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
    ) -> Result<usize, Error>
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
                        return Err(Error::Clone(ErrorInfo {
                            string: String::from("project repo"),
                            err: err.to_string(),
                        }));
                    }

                    if !project.path.join(".projman").exists()
                    {
                        let mut projman_file: fs::File =
                            match fs::File::create_new(project.path.join(".projman"))
                            {
                                Ok(file) => file,
                                Err(err) =>
                                {
                                    return Err(Error::Create(ErrorInfo {
                                        string: String::from(".projman file"),
                                        err: err.to_string(),
                                    }));
                                }
                            };

                        if let Err(err) =
                            projman_file.write_all(project.project_type.to_string().as_bytes())
                        {
                            return Err(Error::WriteTo(ErrorInfo {
                                string: String::from(".projman file"),
                                err: err.to_string(),
                            }));
                        }
                    }

                    let projects_from_json: String = match read_to_string(&projects_path)
                    {
                        Ok(json) => json,
                        Err(err) =>
                        {
                            return Err(Error::Read(ErrorInfo {
                                string: String::from("projects.json"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
                    {
                        Ok(projects) => projects,
                        Err(err) =>
                        {
                            return Err(Error::Parse(ErrorInfo {
                                string: String::from("projects.json"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    projects[index].exists = true;

                    let projects_to_json: String = match serde_json::to_string_pretty(&projects)
                    {
                        Ok(json) => json,
                        Err(err) =>
                        {
                            return Err(Error::Parse(ErrorInfo {
                                string: String::from("projects.json"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
                    {
                        return Err(Error::WriteTo(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
                    };

                    return Ok(index);
                }

                Err(Error::Other(String::from("Error: No prject selected")))
            }
            Err(err) => Err(err),
        }
    }

    pub fn import_project(&mut self) -> Result<(), Error>
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
                    Err(err) =>
                    {
                        return Err(Error::Read(ErrorInfo {
                            string: String::from(".projman file"),
                            err: err.to_string(),
                        }));
                    }
                };

                let repo: String = match Project::get_remote(&path)
                {
                    Ok(url) => url,
                    Err(err) =>
                    {
                        return Err(Error::Fetch(ErrorInfo {
                            string: String::from("remote origin"),
                            err: err.to_string(),
                        }));
                    }
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
                    Err(err) =>
                    {
                        return Err(Error::Read(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
                    }
                };

                let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
                {
                    Ok(projects) => projects,
                    Err(err) =>
                    {
                        return Err(Error::Parse(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
                    }
                };

                projects.push(project);

                let projects_to_json: String = match serde_json::to_string_pretty(&projects)
                {
                    Ok(json) => json,
                    Err(err) =>
                    {
                        return Err(Error::Parse(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
                    }
                };

                if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
                {
                    return Err(Error::WriteTo(ErrorInfo {
                        string: String::from("projects.json"),
                        err: err.to_string(),
                    }));
                };

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn create_project_list_from_json() -> Result<Vec<Project>, Error>
    {
        match AppState::get_config_dir(String::from("projects.json"), Some(String::from("[]")))
        {
            Ok(projects_path) =>
            {
                let projects_from_json: String = match read_to_string(&projects_path)
                {
                    Ok(json) => json,
                    Err(err) =>
                    {
                        return Err(Error::Read(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
                    }
                };

                let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
                {
                    Ok(projects) => projects,
                    Err(err) =>
                    {
                        return Err(Error::Parse(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
                    }
                };

                for project in &mut projects
                {
                    project.exists =
                        project.path.is_dir() && project.path.join(".projman").is_file();
                }

                let projects_to_json: String = match serde_json::to_string_pretty(&projects)
                {
                    Ok(json) => json,
                    Err(err) =>
                    {
                        return Err(Error::Parse(ErrorInfo {
                            string: String::from("projects.json"),
                            err: err.to_string(),
                        }));
                    }
                };

                if let Err(err) = write(&projects_path, projects_to_json.as_bytes())
                {
                    return Err(Error::WriteTo(ErrorInfo {
                        string: String::from("projects.json"),
                        err: err.to_string(),
                    }));
                };

                Ok(projects)
            }
            Err(err) => Err(err),
        }
    }
}
