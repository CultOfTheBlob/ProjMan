use std::{ffi::OsStr, fs::ReadDir, path::PathBuf};

use crate::{
    error::{Error, ErrorInfo},
    state::app_state::AppState,
    templates::template::Template,
};

pub mod template;
pub mod template_config;

#[derive(Debug, Clone, Default)]
pub struct Templates
{
    templates: Vec<Template>,
}

impl Templates
{
    pub fn generate(&self) -> Result<Self, Error>
    {
        let templates_path: PathBuf = AppState::get_config_dir("templates".to_string(), None)?;
        let templates_dir: ReadDir = match templates_path.read_dir()
        {
            Ok(sub_dirs) => sub_dirs,
            Err(err) => return Err(error!(Error::Read, "templates dir", err)),
        };

        let mut templates: Vec<Template> = vec![];
        for entry in templates_dir
        {
            let template_path: PathBuf = match entry
            {
                Ok(entry) => entry.path(),
                Err(err) => return Err(error!(Error::Read, "templates dir", err)),
            };

            if !template_path.is_file() || template_path.extension() != Some(OsStr::new("json"))
            {
                continue;
            }

            let template_name: String = match template_path.with_extension("").iter().next_back()
            {
                Some(component) => component.to_string_lossy().to_string(),
                None => return Err(error!(Error::Read, "template name", "")),
            };

            let template: Template = Template::new(template_name)?;

            templates.push(template);
        }

        Ok(Templates { templates })
    }

    pub fn templates(&self) -> &Vec<Template>
    {
        &self.templates
    }
}
