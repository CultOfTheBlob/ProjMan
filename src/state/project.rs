use std::{
    fs::{create_dir, read_dir, remove_dir},
    io::{self},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::state::{config::Config, project_type::ProjectType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project
{
    pub name: String,
    pub path: PathBuf,
    pub project_type: ProjectType,
}

impl Project
{
    pub fn run(&self) -> io::Result<()>
    {
        self.project_type.run(self)
    }

    pub fn path_is_valid(&self, projects_dir: &str) -> (bool, String)
    {
        let error_message: &str = "Cannot create project in this directory!\n";

        if !self.path.starts_with(projects_dir)
        {
            return (false, format!("{error_message}(not in projects_dir)"));
        }

        if !self.path.has_root()
        {
            return (false, format!("{error_message}(missing root slash)"));
        }

        if self.path.exists()
        {
            match read_dir(&self.path)
            {
                Ok(mut entries) =>
                {
                    if entries.next().is_some()
                    {
                        return (
                            false,
                            format!("{error_message}(dir exists and isnt empty)").to_string(),
                        );
                    }
                }
                Err(_) =>
                {
                    return (
                        false,
                        format!("{error_message}(could not validate dir)").to_string(),
                    );
                }
            }

            return (true, String::new());
        }

        match create_dir(&self.path)
        {
            Ok(_) =>
            {
                if remove_dir(&self.path).is_err()
                {
                    return (
                        false,
                        format!("{error_message}(could not validate dir)").to_string(),
                    );
                }
                (true, String::new())
            }
            Err(_) => (false, format!("{error_message}(permission denied)")),
        }
    }
    pub fn default(config: &Config) -> Self
    {
        let name: &str = "NewProject";

        Self {
            name: String::from(name),
            path: PathBuf::from(&config.general.projects_dir).join(name),
            project_type: ProjectType::default(),
        }
    }
}
