use crate::{
    prelude::*,
    project::{Existant, Nonexistant, Project},
};
use git2::{Config, Cred, FetchOptions, RemoteCallbacks, build::RepoBuilder};
use std::{fs, marker::PhantomData, path::PathBuf};

impl Project<Nonexistant> {
    pub fn clone_repo(&self) -> Result<()> {
        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(|url, username_from_url, allowed| {
            if allowed.is_ssh_key() {
                return Cred::ssh_key_from_agent(username_from_url.unwrap_or_default());
            }

            let config = Config::open_default()?;
            Cred::credential_helper(&config, url, username_from_url)
        });

        let mut fetch = FetchOptions::new();
        fetch.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch);

        builder
            .clone(&self.repo, &self.path)
            .map_err(|err| Error::CloneProjectRepo(err.to_string()))?;

        Ok(())
    }

    pub fn restore(self) -> Result<Project<Existant>> {
        if self.path.exists() {
            let project_path = PathBuf::from(&self.path);

            let project_file_path = project_path.join(Project::<()>::PROJECT_FILE_NAME);

            let project_file_contents = toml::to_string_pretty(&self)
                .map_err(|err| Error::RestoreProject(err.to_string()))?;

            fs::write(project_file_path, project_file_contents)
                .map_err(|err| Error::RestoreProject(err.to_string()))?;
        } else {
            self.clone_repo()?;
        }

        Ok(Project::<Existant> {
            name: self.name,
            path: self.path,
            repo: self.repo,
            license: self.license,
            template_name: self.template_name,
            state: PhantomData,
        })
    }
}
