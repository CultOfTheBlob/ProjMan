use std::{
    fs::{metadata, read_dir},
    io::{self},
    path::PathBuf,
};

use git2::{Cred, FetchOptions, RemoteCallbacks, build::RepoBuilder};
use serde::{Deserialize, Serialize};

use crate::state::{config::Config, project_type::ProjectType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project
{
    pub exists: bool,
    pub name: String,
    pub path: PathBuf,
    pub project_type: ProjectType,
    pub repo: String,
}

impl Project
{
    pub fn run(&self) -> io::Result<()>
    {
        self.project_type.run(self)
    }

    pub fn default(config: &Config) -> Self
    {
        let name: &str = "NewProject";

        Self {
            exists: true,
            name: String::from(name),
            path: PathBuf::from(&config.general.projects_dir).join(name),
            project_type: ProjectType::default(),
            repo: String::new(),
        }
    }

    pub fn clone_repo(&self) -> Result<(), std::io::Error>
    {
        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(|url, username_from_url, allowed| {
            if allowed.is_ssh_key()
            {
                return Cred::ssh_key_from_agent(username_from_url.unwrap());
            }

            let config = git2::Config::open_default()?;
            Cred::credential_helper(&config, url, username_from_url)
        });

        let mut fetch = FetchOptions::new();
        fetch.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch);

        builder.clone(&self.repo, &self.path).map_err(|err| {
            let kind = match err.code()
            {
                git2::ErrorCode::NotFound => std::io::ErrorKind::NotFound,
                git2::ErrorCode::Exists => std::io::ErrorKind::AlreadyExists,
                _ => std::io::ErrorKind::Other,
            };
            std::io::Error::new(kind, err)
        })?;

        Ok(())
    }

    pub fn path_is_valid(&self, projects_dir: &str) -> (bool, String)
    {
        let error_message: &str = "Cannot create project in this directory!\n";

        if !self.path.starts_with(projects_dir)
        {
            return (false, format!("{error_message}(not in projects_dir)"));
        }

        if !self.path.has_root()
        {
            return (false, format!("{error_message}(missing root slash)"));
        }

        if self.path.exists()
        {
            match read_dir(&self.path)
            {
                Ok(mut entries) =>
                {
                    if entries.next().is_some()
                    {
                        return (
                            false,
                            format!("{error_message}(dir exists and isnt empty)").to_string(),
                        );
                    }
                }
                Err(_) =>
                {
                    return (
                        false,
                        format!("{error_message}(could not validate dir)").to_string(),
                    );
                }
            }
        }

        let path = &self.path;

        for p in path.ancestors()
        {
            match metadata(p)
            {
                Ok(metadata) if metadata.permissions().readonly() =>
                {
                    return (
                        false,
                        format!("{error_message}(permission denied)").to_string(),
                    );
                }

                _ => (),
            }
        }

        (true, String::new())
    }
}
