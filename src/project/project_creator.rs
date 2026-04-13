use crate::{
    error::{Error, ErrorInfo},
    project::{Project, ProjmanFile},
    state::app_state::AppState,
};
use iced::futures::future::{self, Ready};
use std::{
    fs::{self, File as FsFile},
    io::Write as _,
    sync::Arc,
};
use tokio::process::Command as TokioCommand;

pub const STEPS: f32 = 8.0;

pub fn create_project_dir(project: &Arc<Project>) -> Ready<Result<String, Error>>
{
    future::ready(match fs::create_dir_all(&project.path)
    {
        Ok(()) => Ok(String::from("Created project dir...")),
        Err(err) => Err(error!(Error::Create, "project dir", err)),
    })
}

pub fn clone_project_repo(project: &Arc<Project>) -> Ready<Result<String, Error>>
{
    future::ready(match project.clone_repo()
    {
        Ok(()) => Ok(String::from("Cloned project repo...")),

        Err(err) =>
        {
            let _ = fs::remove_dir(&project.path);
            Err(error!(Error::Clone, "project repo", err))
        }
    })
}

pub fn create_projman_file(project: &Arc<Project>) -> Ready<Result<String, Error>>
{
    match FsFile::create_new(project.path.join("projman.toml"))
    {
        Ok(mut file) =>
        {
            let projman_file = ProjmanFile {
                name: project.name.clone(),
                template_name: project.template_name.clone(),
                repo: project.repo.clone(),
                license: project.license.clone(),
            };

            let project_to_toml = match toml::to_string_pretty(&projman_file)
            {
                Ok(string) => string,
                Err(err) => return future::ready(Err(error!(Error::Parse, "project", err))),
            };

            if let Err(err) = file.write_all(project_to_toml.as_bytes())
            {
                return future::ready(Err(error!(Error::Write, "projman.toml", err)));
            }
        }
        Err(err) =>
        {
            return future::ready(Err(error!(Error::Create, "projman.toml", err)));
        }
    }

    future::ready(Ok(String::from("Created projman.toml...")))
}

pub fn create_dir_structure(project: &Arc<Project>) -> Ready<Result<String, Error>>
{
    for dir in &project.template.config().dir_structure
    {
        let dirs = dir.parse(&project.path);

        for dir in &dirs
        {
            if let Err(err) = fs::create_dir_all(dir)
            {
                return future::ready(Err(error!(Error::Create, "directory structure", err)));
            }
        }
    }

    future::ready(Ok(String::from("Created project directory structure...")))
}

pub fn create_project_files(project: &Arc<Project>) -> Ready<Result<String, Error>>
{
    for file in &project.template.config().files
    {
        if let Err(err) = fs::write(
            project.path.join(&file.path),
            file.formatted(&project.name, &project.repo, &project.license),
        )
        {
            return future::ready(Err(error!(Error::Create, "project files", err)));
        }
    }

    future::ready(Ok(String::from("Created project files...")))
}

pub async fn execute_build_command(project: Arc<Project>, index: usize) -> Result<String, Error>
{
    let command = &project.template.config().build[index];

    match TokioCommand::new(&command.program)
        .args(&command.args)
        .current_dir(&project.path)
        .kill_on_drop(true)
        .status()
        .await
    {
        Ok(_) => Ok(format!("Executed [{command}]...")),
        Err(err) => Err(error!(Error::Run, command, err)),
    }
}

pub fn commit_projman_init(project: &Arc<Project>) -> Ready<Result<String, Error>>
{
    match project.init_commit()
    {
        Ok(()) => future::ready(Ok(String::from("Committed ProjMan init..."))),

        Err(err) =>
        {
            let _ = fs::remove_dir(&project.path);
            future::ready(Err(error!(Error::Commit, "ProjMan init", err)))
        }
    }
}

pub fn add_project_to_json(project: &Arc<Project>) -> Ready<Result<Vec<Project>, Error>>
{
    future::ready((|| {
        let config_path = AppState::get_config_dir("projects.json", None)?;

        let projects_from_json = match fs::read_to_string(&config_path)
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

        let project = (*Arc::clone(project)).clone();

        projects.push(project);

        let projects_to_json = match serde_json::to_string_pretty(&projects)
        {
            Ok(json) => json,
            Err(err) =>
            {
                return Err(error!(Error::Parse, "projects.json", err));
            }
        };

        if let Err(err) = fs::write(&config_path, projects_to_json.as_bytes())
        {
            return Err(error!(Error::Write, "projects.json", err));
        }

        Ok(projects)
    })())
}
