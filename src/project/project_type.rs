use std::{
    fmt::{self, Display},
    path::PathBuf,
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

    pub fn template(&self) -> Result<TemplateConfig, Error>
    {
        match self
        {
            ProjectType::Base => Base::template(),
        }
    }

    pub fn run(&self, project: &Project) -> Result<(), Error>
    {
        match self
        {
            ProjectType::Base =>
            {
                for command in &Base::template()?.run
                {
                    if let Err(err) = process::Command::new(&command.program)
                        .args(&command.args)
                        .current_dir(&project.path)
                        .spawn()
                    {
                        return Err(Error::Run(ErrorInfo {
                            string: command.to_string(),
                            err: err.to_string(),
                        }));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn icon(&self) -> PathBuf
    {
        let icons_path: PathBuf = PathBuf::from("icons");

        match self
        {
            ProjectType::Base => icons_path.join("base.svg"),
        }
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
