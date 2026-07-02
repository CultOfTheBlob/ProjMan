use std::iter;

use crate::project::Project;
use git2::{
    Config, Cred, Error as Git2Error, FetchOptions, IndexAddOption, RemoteCallbacks, Repository,
    build::RepoBuilder,
};

impl Project {
    pub fn clone_repo(&self) -> Result<(), Git2Error> {
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

        builder.clone(&self.repo, &self.path)?;

        Ok(())
    }

    pub fn init_commit(&self) -> Result<(), Git2Error> {
        let project_repo = Repository::open(&self.path)?;

        let mut index = project_repo.index()?;

        index.add_all(iter::once(&"*"), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let signature = project_repo.signature()?;
        let tree = project_repo.find_tree(index.write_tree()?)?;
        let parent_commit = project_repo.head()?.peel_to_commit()?;

        project_repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initialized ProjMan project",
            &tree,
            &[&parent_commit],
        )?;

        Ok(())
    }
}
