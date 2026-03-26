use std::{
    fmt::{self, Display},
    fs::read_to_string,
    path::{Path, PathBuf},
    process,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
    templates::template_config::TemplateConfig,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Template
{
    name: String,
    config: TemplateConfig,
    template_path: PathBuf,
    icon_path: PathBuf,
}

impl Template
{
    pub fn new(name: String) -> Result<Self, Error>
    {
        let template_path: PathBuf =
            AppState::get_config_dir(format!("templates/{name}.json"), None)?;

        let icon_path: PathBuf = AppState::get_config_dir(format!("icons/{name}.svg"), None)?;

        let template_string: String = match read_to_string(&template_path)
        {
            Ok(string) => string,
            Err(err) => return Err(error!(Error::Read, format!("{name} template"), err)),
        };

        let config: TemplateConfig = match serde_json::from_str(&template_string)
        {
            Ok(template_config) => template_config,
            Err(err) => return Err(error!(Error::Parse, format!("{name} template"), err)),
        };

        Ok(Template {
            name,
            config,
            template_path,
            icon_path,
        })
    }

    pub fn run(&self, project: &Project) -> Result<(), Error>
    {
        for command in &self.config.run
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
        self.config
            .included_paths
            .iter()
            .map(|path| root.join(path))
            .collect()
    }

    pub fn excluded_paths(&self) -> Vec<&str>
    {
        self.config()
            .excluded_paths
            .iter()
            .map(|s| s.as_str())
            .collect()
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }

    pub fn config(&self) -> &TemplateConfig
    {
        &self.config
    }

    pub fn icon_path(&self) -> &PathBuf
    {
        &self.icon_path
    }
}

impl Display for Template
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Template
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        Template::new(s.to_string())
    }
}
