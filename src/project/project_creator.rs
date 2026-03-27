use std::{
    fs::{self, create_dir, create_dir_all, read_to_string, remove_dir, write},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use askalono::{Store, TextData};
use tokio::process;

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
    templates::template_config::Command,
};

pub const STEPS: f32 = 8.0;

pub async fn create_project_dir(project: Arc<Project>) -> Result<String, Error>
{
    match create_dir_all(&project.path)
    {
        Ok(_) => Ok("Created project dir...".to_string()),
        Err(err) => Err(error!(Error::Create, "project dir", err)),
    }
}

pub async fn clone_project_repo(project: Arc<Project>) -> Result<String, Error>
{
    match project.clone_repo()
    {
        Ok(_) => Ok("Cloned project repo...".to_string()),

        Err(err) =>
        {
            let _ = remove_dir(&project.path);
            Err(error!(Error::Clone, "project repo", err))
        }
    }
}

pub async fn create_projman_file(project: Arc<Project>) -> Result<String, Error>
{
    match fs::File::create_new(project.path.join("projman.toml"))
    {
        Ok(mut file) =>
        {
            let project_to_toml: String = match toml::to_string_pretty(&project)
            {
                Ok(string) => string,
                Err(err) => return Err(error!(Error::Parse, "project", err)),
            };

            if let Err(err) = file.write_all(project_to_toml.as_bytes())
            {
                return Err(error!(Error::Write, "projman.toml", err));
            }
        }
        Err(err) =>
        {
            return Err(error!(Error::Create, "projman.toml", err));
        }
    };

    Ok("Created projman.toml...".to_string())
}

pub async fn create_dir_structure(project: Arc<Project>) -> Result<String, Error>
{
    for dir in &project.template.config().dir_structure
    {
        let dirs: Vec<PathBuf> = dir.parse(&project.path);

        for dir in &dirs
        {
            if let Err(err) = create_dir(dir)
            {
                return Err(error!(Error::Create, "directory structure", err));
            }
        }
    }

    Ok("Created project directory structure...".to_string())
}

pub async fn create_project_files(project: Arc<Project>) -> Result<String, Error>
{
    for file in &project.template.config().files
    {
        if let Err(err) = write(project.path.join(&file.path), &file.content)
        {
            return Err(error!(Error::Create, "project files", err));
        };
    }

    Ok("Created project files...".to_string())
}

pub async fn execute_build_command(project: Arc<Project>, index: usize) -> Result<String, Error>
{
    let command: &Command = &project.template.config().build[index];

    match process::Command::new(&command.program)
        .args(&command.args)
        .current_dir(&project.path)
        .kill_on_drop(true)
        .status()
        .await
    {
        Ok(_) => Ok(format!("Executed [{}]...", command)),
        Err(err) => Err(error!(Error::Run, command, err)),
    }
}

pub async fn commit_projman_init(project: Arc<Project>) -> Result<String, Error>
{
    match project.init_commit()
    {
        Ok(_) => Ok("Committed ProjMan init...".to_string()),

        Err(err) =>
        {
            let _ = remove_dir(&project.path);
            Err(error!(Error::Commit, "ProjMan init", err))
        }
    }
}

pub async fn add_project_to_json(project: Arc<Project>) -> Result<Vec<Project>, Error>
{
    let config_path: PathBuf = AppState::get_config_dir(String::from("projects.json"), None)?;

    let projects_from_json: String = match read_to_string(&config_path)
    {
        Ok(json) => json,
        Err(err) =>
        {
            return Err(error!(Error::Read, "projects.json", err));
        }
    };

    let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
    {
        Ok(projects) => projects,
        Err(err) =>
        {
            return Err(error!(Error::Parse, "projects.json", err));
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
                    return Err(error!(Error::Parse, "project license", err));
                }
            };

            let license_path: PathBuf = project.path.join("LICENSE");

            let license_contents: String = match read_to_string(&license_path)
            {
                Ok(contents) => contents,
                Err(err) =>
                {
                    return Err(error!(Error::Read, "LICENSE file", err));
                }
            };

            store
                .analyze(&TextData::from(license_contents.as_str()))
                .name
                .to_string()
        };

        let project: Project = (*project).clone();
        Project { license, ..project }
    };

    projects.push(project);

    let projects_to_json: String = match serde_json::to_string_pretty(&projects)
    {
        Ok(json) => json,
        Err(err) =>
        {
            return Err(error!(Error::Parse, "projects.json", err));
        }
    };

    if let Err(err) = write(&config_path, projects_to_json.as_bytes())
    {
        return Err(error!(Error::Write, "projects.json", err));
    };

    Ok(projects)
}
