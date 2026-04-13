use crate::{
    error::{Error, ErrorInfo},
    state::app_state::AppState,
    templates::template::Template,
};
use std::{collections::HashMap, ffi::OsStr, sync::Arc};

pub mod template;
pub mod template_config;

#[derive(Debug, Clone, Default)]
pub struct Templates
{
    templates: HashMap<String, Arc<Template>>,
}

impl Templates
{
    pub fn generate() -> Result<Self, Error>
    {
        let templates_path = AppState::get_config_dir("templates", None)?;
        let templates_dir = match templates_path.read_dir()
        {
            Ok(sub_dirs) => sub_dirs,
            Err(err) => return Err(error!(Error::Read, "templates dir", err)),
        };

        let mut templates: HashMap<String, Arc<Template>> = HashMap::new();
        for entry in templates_dir
        {
            let template_path = match entry
            {
                Ok(entry) => entry.path(),
                Err(err) => return Err(error!(Error::Read, "templates dir", err)),
            };

            if !template_path.is_file() || template_path.extension() != Some(OsStr::new("json"))
            {
                continue;
            }

            let template_name = match template_path.with_extension("").iter().next_back()
            {
                Some(component) => component.to_string_lossy().to_string(),
                None => return Err(error!(Error::Read, "template name", "")),
            };

            let template = Template::new(&template_name)?;

            templates.insert(template_name, Arc::new(template));
        }

        Ok(Self { templates })
    }

    pub fn template_names(&self) -> Vec<String>
    {
        self.templates.keys().cloned().collect()
    }

    pub fn get(&self, name: &str) -> Result<Arc<Template>, Error>
    {
        match self.templates.get(name).cloned()
        {
            Some(template) => Ok(template),
            None => Err(error!(Error::Find, "template", "")),
        }
    }
}
