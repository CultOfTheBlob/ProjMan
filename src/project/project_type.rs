use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
    process,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    templates::{Template, TemplateConfig, base::Base},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ProjectType
{
    #[default]
    Base,
}

impl ProjectType
{
    pub const ALL: [ProjectType; 1] = [ProjectType::Base];

    fn template(&self) -> impl Template
    {
        match self
        {
            ProjectType::Base => Base,
        }
    }

    pub fn template_config(&self) -> Result<TemplateConfig, Error>
    {
        self.template().template()
    }

    pub fn run(&self, project: &Project) -> Result<(), Error>
    {
        for command in &self.template_config()?.run
        {
            if let Err(err) = process::Command::new(&command.program)
                .args(&command.args)
                .current_dir(&project.path)
                .spawn()
            {
                return Err(error!(Error::Run, command.to_string(), err));
            }
        }

        Ok(())
    }

    pub fn included_paths(&self, root: &Path) -> Vec<PathBuf>
    {
        self.template()
            .included_paths()
            .iter()
            .map(|path| root.join(path))
            .collect()
    }

    pub fn excluded_paths(&self) -> &'static [&'static str]
    {
        self.template().excluded_paths()
    }

    pub fn icon(&self) -> PathBuf
    {
        let icons_path: PathBuf = PathBuf::from("icons");

        icons_path.join(self.template().icon_path())
    }
}

impl Display for ProjectType
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> fmt::Result
    {
        write!(
            formatter,
            "{}",
            match self
            {
                ProjectType::Base => "Base",
            }
        )
    }
}

impl FromStr for ProjectType
{
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        match string
        {
            "Base" => Ok(ProjectType::Base),
            _ => Err(Error::Other(String::from("Project type is not valid!"))),
        }
    }
}
