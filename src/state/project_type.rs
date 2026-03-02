use std::{
    fmt::{self, Display},
    io::{self},
    process,
};

use serde::{Deserialize, Serialize};

use crate::state::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ProjectType
{
    #[default]
    Base,
}

impl ProjectType
{
    pub const ALL: [ProjectType; 1] = [ProjectType::Base];

    pub fn run(&self, project: &Project) -> io::Result<()>
    {
        match self
        {
            ProjectType::Base =>
            {
                process::Command::new("kitty")
                    .arg("--detach")
                    .current_dir(&project.path)
                    .spawn()?;
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
