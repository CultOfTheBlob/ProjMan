use std::{
    fs::{File, create_dir_all, read_to_string},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub mod base;

pub trait Template
{
    fn template() -> Result<TemplateConfig, std::io::Error>
    {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
        {
            create_dir_all(proj_dirs.config_dir())?;

            let template_path: PathBuf = proj_dirs.config_dir().join(Self::template_path());

            if !template_path.is_file()
            {
                let default_template: TemplateConfig = Self::default();

                File::create(&template_path)?.write_all(
                    serde_json::to_string_pretty(&default_template)
                        .unwrap()
                        .as_bytes(),
                )?;

                return Ok(default_template);
            }

            let template: TemplateConfig = serde_json::from_str(&read_to_string(template_path)?)?;

            return Ok(template);
        }

        Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Could not find config folder",
        ))
    }

    fn default() -> TemplateConfig;

    fn template_path() -> &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateConfig
{
    pub dir_structure: Option<Folder>,
    pub run: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder
{
    pub name: String,
    pub sub_dirs: Vec<Folder>,
}

impl Folder
{
    pub fn parse(&self, root: &Path) -> Vec<PathBuf>
    {
        let mut dirs: Vec<PathBuf> = vec![root.join(&self.name)];

        for dir in &self.sub_dirs
        {
            let mut sub_dirs = dir.parse(&root.join(&self.name));

            dirs.append(&mut sub_dirs);
        }

        dirs
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command
{
    pub program: String,
    pub args: Vec<String>,
}
