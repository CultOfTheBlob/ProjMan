use std::{
    fs::{self, create_dir, create_dir_all, read_to_string, remove_dir, write},
    io::Write,
    path::PathBuf,
    process::{self},
};

use askalono::{Store, TextData};

use crate::{
    error::{Error, ErrorInfo},
    project::{Project, project_type::ProjectType},
    state::app_state::AppState,
    templates::{Command, File, Folder},
};

pub const STEPS: f32 = 8.0;

pub async fn create_project_dir(project_path: PathBuf) -> Result<String, Error>
{
    match create_dir_all(&project_path)
    {
        Ok(_) => Ok("Created project dir...".to_string()),
        Err(err) => Err(Error::Create(ErrorInfo {
            string: String::from("project dir"),
            err: err.to_string(),
        })),
    }
}

pub async fn clone_project_repo(project: Project) -> Result<String, Error>
{
    match project.clone_repo()
    {
        Ok(_) => Ok("Cloned project repo...".to_string()),

        Err(err) =>
        {
            let _ = remove_dir(&project.path);
            Err(Error::Clone(ErrorInfo {
                string: String::from("project repo"),
                err: err.to_string(),
            }))
        }
    }
}

pub async fn create_projman_file(
    project_path: PathBuf,
    project_type: ProjectType,
) -> Result<String, Error>
{
    match fs::File::create_new(project_path.join(".projman"))
    {
        Ok(mut file) =>
        {
            if let Err(err) = file.write_all(project_type.to_string().as_bytes())
            {
                return Err(Error::Create(ErrorInfo {
                    string: String::from(".projman file"),
                    err: err.to_string(),
                }));
            }
        }
        Err(err) =>
        {
            return Err(Error::Create(ErrorInfo {
                string: String::from(".projman file"),
                err: err.to_string(),
            }));
        }
    };

    Ok("Created .projman file...".to_string())
}

pub async fn create_dir_structure(
    project_dir_structure: Vec<Folder>,
    project_path: PathBuf,
) -> Result<String, Error>
{
    for dir in &project_dir_structure
    {
        let dirs: Vec<PathBuf> = dir.parse(&project_path);

        for dir in &dirs
        {
            if let Err(err) = create_dir(dir)
            {
                return Err(Error::Create(ErrorInfo {
                    string: String::from("directory structure"),
                    err: err.to_string(),
                }));
            }
        }
    }

    Ok("Created project directory structure...".to_string())
}

pub async fn create_project_files(
    project_files: Vec<File>,
    project_path: PathBuf,
) -> Result<String, Error>
{
    for file in &project_files
    {
        if let Err(err) = write(project_path.join(&file.path), &file.content)
        {
            return Err(Error::Create(ErrorInfo {
                string: String::from("project files"),
                err: err.to_string(),
            }));
        };
    }

    Ok("Created project files...".to_string())
}

pub async fn execute_build_command(command: Command, project_path: PathBuf)
-> Result<String, Error>
{
    match process::Command::new(&command.program)
        .args(&command.args)
        .current_dir(&project_path)
        .status()
    {
        Ok(_) => Ok(format!("Executed [{}]...", command)),
        Err(err) => Err(Error::Run(ErrorInfo {
            string: command.to_string(),
            err: err.to_string(),
        })),
    }
}

pub async fn commit_projman_init(project: Project) -> Result<String, Error>
{
    match project.init_commit()
    {
        Ok(_) => Ok("Committed ProjMan init...".to_string()),

        Err(err) =>
        {
            let _ = remove_dir(&project.path);
            Err(Error::Commit(ErrorInfo {
                string: String::from("ProjMan init"),
                err: err.to_string(),
            }))
        }
    }
}

pub async fn add_project_to_json(project: Project) -> Result<Vec<Project>, Error>
{
    match AppState::get_config_dir(String::from("projects.json"), None)
    {
        Ok(config_path) =>
        {
            let projects_from_json: String = match read_to_string(&config_path)
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

            let project: Project = {
                let license: String = {
                    let store: Store = match Store::from_cache(
                        &include_bytes!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/cache/license.cache.zstd"
                        ))[..],
                    )
                    {
                        Ok(store) => store,
                        Err(err) =>
                        {
                            return Err(Error::Fetch(ErrorInfo {
                                string: String::from("project license"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    let license_path: PathBuf = project.path.join("LICENSE");

                    let license_contents: String = match read_to_string(&license_path)
                    {
                        Ok(contents) => contents,
                        Err(err) =>
                        {
                            return Err(Error::Read(ErrorInfo {
                                string: String::from("LICENSE file"),
                                err: err.to_string(),
                            }));
                        }
                    };

                    store
                        .analyze(&TextData::from(license_contents.as_str()))
                        .name
                        .to_string()
                };

                Project { license, ..project }
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

            if let Err(err) = write(&config_path, projects_to_json.as_bytes())
            {
                return Err(Error::Write(ErrorInfo {
                    string: String::from("projects.json"),
                    err: err.to_string(),
                }));
            };

            Ok(projects)
        }
        Err(err) => Err(err),
    }
}
