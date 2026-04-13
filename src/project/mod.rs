use crate::{error::Error, state::config::Config, templates::template::Template};
use askalono::{Store, TextData};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Arc};

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
        self.template.icon_path().clone()
    }

    pub fn default(config: &Config) -> Self
    {
        let name = "NewProject";

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
        let license = {
            #[expect(clippy::large_include_file)]
            let cache = &include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/cache/license.cache.zstd"
            ))[..];

            let Ok(store) = Store::from_cache(cache)
            else
            {
                return self;
            };

            let license_path = self.path.join("LICENSE");
            let Ok(license_contents) = fs::read_to_string(&license_path)
            else
            {
                return self;
            };
            store
                .analyze(&TextData::from(license_contents.as_str()))
                .name
                .to_owned()
        };

        Self { license, ..self }
    }

    pub fn is_outdated(&self) -> bool
    {
        let project_files = &self.template.config().files;

        for file in project_files
        {
            if !file.tracked
            {
                continue;
            }

            let path = PathBuf::from(&self.path).join(&file.path);

            let Ok(file_contents) = fs::read(&path)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjmanFile
{
    pub name: String,
    pub template_name: String,
    pub repo: String,
    pub license: String,
}
