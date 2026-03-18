use std::{
    fmt::{self, Display},
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

    pub fn run(&self, project: &Project) -> Result<(), String>
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
                        return Err(format!(
                            "Error: Could not run command [{command:?}] ({err})"
                        ));
                    }
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
