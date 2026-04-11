use crate::error::ErrorInfo;
use std::{
    fs::{read, read_to_string},
    path::PathBuf,
    sync::Arc,
};

use askalono::{Store, TextData};
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

    pub fn license(self) -> Self
    {
        let license: String = {
            let store: Store = match Store::from_cache(
                &include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/cache/license.cache.zstd"
                ))[..],
            )
            {
                Ok(store) => store,
                Err(_) =>
                {
                    return self;
                }
            };

            let license_path: PathBuf = self.path.join("LICENSE");

            let license_contents: String = match read_to_string(&license_path)
            {
                Ok(contents) => contents,
                Err(_) =>
                {
                    return self;
                }
            };

            store
                .analyze(&TextData::from(license_contents.as_str()))
                .name
                .to_string()
        };

        Project { license, ..self }
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
