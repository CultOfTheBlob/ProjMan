use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::{Path, PathBuf},
};

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
    pub sub_dirs: Vec<Self>,
}

impl Folder
{
    pub fn parse(&self, root: &Path) -> Vec<PathBuf>
    {
        let mut dirs = vec![root.join(&self.name)];

        for dir in &self.sub_dirs
        {
            let mut sub_dirs = dir.parse(&root.join(&self.name));

            dirs.append(&mut sub_dirs);
        }

        dirs
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct File
{
    pub path: String,
    pub content: String,
    pub tracked: bool,
}

impl File
{
    #[expect(clippy::literal_string_with_formatting_args)]
    pub fn formatted(&self, name: &str, repo: &str, license: &str) -> String
    {
        self.content
            .replace("#{name}", name)
            .replace("#{repo}", repo)
            .replace("#{license}", license)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Command
{
    pub program: String,
    pub args: Vec<String>,
}

impl Display for Command
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
    {
        write!(formatter, "{} {}", self.program, &self.args.join(" "))
    }
}
