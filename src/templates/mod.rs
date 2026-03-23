use std::{
    fmt::{self, Display},
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, ErrorInfo},
    state::app_state::AppState,
};

pub mod base;

pub trait Template
{
    const TEMPLATE_PATH: &str;

    fn template(&self) -> Result<TemplateConfig, Error>
    {
        let default_template =
            Some(serde_json::to_string_pretty(&self.default()).unwrap_or(String::from("{}")));

        match AppState::get_config_dir(String::from(Self::TEMPLATE_PATH), default_template)
        {
            Ok(template_path) =>
            {
                let template: TemplateConfig = match read_to_string(template_path)
                {
                    Ok(string) => match serde_json::from_str(&string)
                    {
                        Ok(it) => it,
                        Err(err) =>
                        {
                            return Err(Error::Parse(ErrorInfo {
                                string: String::from("template"),
                                err: err.to_string(),
                            }));
                        }
                    },
                    Err(err) =>
                    {
                        return Err(Error::Read(ErrorInfo {
                            string: String::from("template"),
                            err: err.to_string(),
                        }));
                    }
                };

                Ok(template)
            }
            Err(err) => Err(err),
        }
    }

    fn default(&self) -> TemplateConfig;

    fn included_paths(&self) -> &'static [&'static str];
    fn excluded_paths(&self) -> &'static [&'static str];

    fn icon_path(&self) -> &'static str;
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

impl Display for Command
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> fmt::Result
    {
        write!(formatter, "{} {}", self.program, &self.args.join(" "))
    }
}
