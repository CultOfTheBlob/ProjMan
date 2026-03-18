use std::{
    fmt::{self, Display},
    io::{self},
    process,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    state::project::Project,
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

    pub fn template(&self) -> Result<TemplateConfig, String>
    {
        match self
        {
            ProjectType::Base => Base::template(),
        }
    }

    pub fn run(&self, project: &Project) -> io::Result<()>
    {
        match self
        {
            ProjectType::Base =>
            {
                for command in &Base::template()?.run
                {
                    process::Command::new(&command.program)
                        .args(&command.args)
                        .current_dir(&project.path)
                        .spawn()?;
                }
            }
        }

        Ok(())
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
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        match string
        {
            "Base" => Ok(ProjectType::Base),
            _ => Err(String::from("Project type is not valid!")),
        }
    }
}
