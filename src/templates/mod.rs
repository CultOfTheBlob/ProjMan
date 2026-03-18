use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::state::app_state::AppState;

pub mod base;

pub trait Template
{
    fn template() -> Result<TemplateConfig, String>
    {
        let default_template =
            Some(serde_json::to_string_pretty(&Self::default()).unwrap_or(String::from("{}")));

        match AppState::get_config_dir(String::from(Self::template_path()), default_template)
        {
            Ok(template_path) =>
            {
                let template: TemplateConfig = match read_to_string(template_path)
                {
                    Ok(string) => match serde_json::from_str(&string)
                    {
                        Ok(it) => it,
                        Err(err) => return Err(format!("Error: Could not parse template ({err})")),
                    },
                    Err(err) => return Err(format!("Error: Could not read template ({err})")),
                };

                Ok(template)
            }
            Err(err) => Err(err),
        }
    }

    fn default() -> TemplateConfig;

    fn template_path() -> &'static str;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateConfig
{
    pub dir_structure: Vec<Folder>,
    pub files: Vec<File>,
    pub build: Vec<Command>,
    pub run: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File
{
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command
{
    pub program: String,
    pub args: Vec<String>,
}
