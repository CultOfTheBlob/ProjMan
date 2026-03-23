use std::{
    collections::BTreeMap,
    fs::{metadata, read, read_dir},
    path::PathBuf,
};

use git2::{
    Cred, FetchOptions, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature,
    build::RepoBuilder,
};
use serde::{Deserialize, Serialize};
use tokei::{LanguageType, Languages};

use crate::{error::Error, state::config::Config};
use crate::{project::project_type::ProjectType, templates::File};

pub mod project_creator;
pub mod project_type;

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
    pub fn run(&self) -> Result<(), Error>
    {
        self.project_type.run(self)
    }

    pub fn icon(&self) -> PathBuf
    {
        self.project_type.icon()
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

    pub fn info(&self) -> Option<ProjectInfo>
    {
        let mut languages = Languages::new();
        languages.get_statistics(
            &self.project_type.included_paths(&self.path),
            self.project_type.excluded_paths(),
            &tokei::Config::default(),
        );

        let mut line_count: usize = 0;
        for language in languages.values()
        {
            line_count += language.code;
        }

        let mut language_percentage: Vec<(LanguageType, f64)> = vec![];
        for (language_type, language) in languages
        {
            let percentage: f64 = (language.code as f64 / line_count as f64) * 100.0;

            language_percentage.push((language_type, percentage));
        }
        language_percentage
            .sort_by(|l, p| p.1.partial_cmp(&l.1).unwrap_or(std::cmp::Ordering::Equal));

        Some(ProjectInfo {
            line_count,
            language_percentage,
        })
    }

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

    pub fn path_is_valid(&self, projects_dir: &str) -> Result<(), String>
    {
        let error_message: &str = "Cannot create project in this directory!\n";

        if !self.path.has_root()
        {
            return Err(format!("{error_message}(missing root slash)"));
        }

        if !self.path.starts_with(projects_dir)
        {
            return Err(format!("{error_message}(not in projects_dir)"));
        }

        if self.path.exists()
        {
            match read_dir(&self.path)
            {
                Ok(mut entries) =>
                {
                    if entries.next().is_some()
                    {
                        return Err(format!("{error_message}(dir exists and isnt empty)"));
                    }
                }
                Err(_) =>
                {
                    return Err(format!("{error_message}(could not validate dir)"));
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
                    return Err(format!("{error_message}(permission denied)"));
                }

                _ => (),
            }
        }

        Ok(())
    }

    pub fn is_outdated(&self) -> bool
    {
        let project_files: Vec<File> = match self.project_type.template()
        {
            Ok(template) => template.files,
            Err(_) => return false,
        };

        for file in &project_files
        {
            let path: PathBuf = PathBuf::from(&self.path).join(&file.path);

            let Ok(file_contents) = read(&path)
            else
            {
                return false;
            };

            if file.content.as_bytes() == file_contents
            {
                return false;
            }
        }

        true
    }

    pub fn is_project_path(path: PathBuf) -> bool
    {
        if path.join(".projman").is_file()
        {
            return true;
        }

        false
    }
}

#[derive(Debug)]
pub struct ProjectInfo
{
    pub line_count: usize,
    pub language_percentage: Vec<(LanguageType, f64)>,
}
