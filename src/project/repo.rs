use std::path::PathBuf;

use git2::{
    Cred, FetchOptions, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature,
    build::RepoBuilder,
};

use crate::project::Project;

impl Project
{
    pub fn get_remote(path: &PathBuf) -> Result<String, git2::Error>
    {
        let project_repo: Repository = Repository::open(path)?;

        match project_repo.find_remote("origin")
        {
            Ok(remote) =>
            {
                if let Some(url) = remote.url()
                {
                    Ok(url.to_string())
                }
                else
                {
                    Ok(String::new())
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn clone_repo(&self) -> Result<(), git2::Error>
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

        builder.clone(&self.repo, &self.path)?;

        Ok(())
    }

    pub fn init_commit(&self) -> Result<(), git2::Error>
    {
        let project_repo: Repository = Repository::open(&self.path)?;

        let mut index = project_repo.index()?;

        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let signature: Signature = project_repo.signature()?;
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

        let head = project_repo.head()?;
        let branch = head.shorthand().unwrap();
        let refspec = format!("refs/heads/{0}:refs/heads/{0}", branch);

        let mut remote = project_repo.find_remote("origin")?;

        let mut callbacks = RemoteCallbacks::new();

        callbacks.credentials(|url, username_from_url, allowed| {
            if allowed.is_ssh_key()
            {
                return Cred::ssh_key_from_agent(username_from_url.unwrap());
            }

            let config = git2::Config::open_default()?;
            Cred::credential_helper(&config, url, username_from_url)
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        remote.push(&[&refspec], Some(&mut push_options))?;

        Ok(())
    }
}
