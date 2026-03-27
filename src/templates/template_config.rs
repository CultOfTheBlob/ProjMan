use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TemplateConfig
{
    pub dir_structure: Vec<Folder>,
    pub files: Vec<File>,
    pub build: Vec<Command>,
    pub run: Vec<Command>,
    pub included_paths: Vec<String>,
    pub excluded_paths: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct File
{
    pub path: String,
    pub content: String,
    pub tracked: bool,
}

impl File
{
    pub fn formatted(&self, name: &str, repo: &str) -> String
    {
        self.content
            .replace("#{name}", name)
            .replace("#{repo}", repo)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
