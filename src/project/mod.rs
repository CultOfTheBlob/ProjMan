use std::{fs::read, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    state::config::Config,
    templates::{template::Template, template_config::File},
};

pub mod project_creator;

mod info;
mod path;
mod repo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project
{
    pub exists: bool,
    pub name: String,
    pub path: PathBuf,
    pub template_name: String,
    #[serde(skip)]
    pub template: Arc<Template>,
    pub repo: String,
    pub license: String,
}

impl Project
{
    pub fn run(&self) -> Result<(), Error>
    {
        self.template.run(self)
    }

    pub fn icon(&self) -> PathBuf
    {
        self.template.icon_path().to_path_buf()
    }

    pub fn default(config: &Config) -> Self
    {
        let name: &str = "NewProject";

        Self {
            exists: true,
            name: String::from(name),
            path: PathBuf::from(&config.general.projects_dir).join(name),
            template_name: String::default(),
            template: Arc::new(Template::default()),
            repo: String::new(),
            license: String::new(),
        }
    }

    pub fn is_outdated(&self) -> bool
    {
        let project_files: &Vec<File> = &self.template.config().files;

        for file in project_files
        {
            if !file.tracked
            {
                continue;
            }

            let path: PathBuf = PathBuf::from(&self.path).join(&file.path);

            let Ok(file_contents) = read(&path)
            else
            {
                return true;
            };

            if file.content.as_bytes() != file_contents
            {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjmanFile
{
    pub name: String,
    pub template_name: String,
    pub repo: String,
    pub license: String,
}
