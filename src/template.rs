use crate::{
    config_dir::ConfigDir, prelude::*, template::template_config::TemplateConfig,
};
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command as StdCommand,
};

mod project_context;

pub mod command;
pub mod file;
pub mod folder;
pub mod template_config;

pub fn load_templates() -> Result<BTreeMap<String, Template>> {
    let templates_dir_path = ConfigDir::Templates.get_file(None)?;

    let dir_entries = templates_dir_path
        .read_dir()
        .map_err(|err| Error::ReadTemplatesDir(err.to_string()))?;

    let mut templates = BTreeMap::new();

    for entry in dir_entries {
        let entry_path = entry
            .map_err(|err| Error::ReadTemplatesDir(err.to_string()))?
            .path();

        if !entry_path.is_dir() {
            continue;
        }

        if let Some(template_name) = entry_path.file_name().and_then(OsStr::to_str) {
            let template = Template::load(template_name)?;

            templates.insert(template_name.to_string(), template);
        }
    }

    Ok(templates)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    pub name: String,
    pub config: TemplateConfig,
    pub icon_path: PathBuf,
}

impl Template {
    pub fn load(name: &str) -> Result<Self> {
        let base_dir = ConfigDir::Template(name.to_string()).get_file(None)?;
        let config_path = base_dir.join("template.yaml");
        let icon_path = base_dir.join("icon.svg");

        let config_str = fs::read_to_string(&config_path)
            .map_err(|err| Error::ReadTemplate(name.to_string(), err.to_string()))?;

        let config = serde_yaml::from_str(&config_str)
            .map_err(|err| Error::ParseTemplate(name.to_string(), err.to_string()))?;

        Ok(Self {
            name: name.to_string(),
            config,
            icon_path,
        })
    }

    pub fn run(&self, project_path: &Path) -> Result<()> {
        for command in &self.config.run {
            StdCommand::new(&command.program)
                .args(&command.args)
                .current_dir(project_path)
                .spawn()
                .map_err(|err| Error::RunCommand(command.to_string(), err.to_string()))?;
        }
        Ok(())
    }

    pub fn included_paths(&self, root: &Path) -> Vec<PathBuf> {
        self.config
            .included_paths
            .iter()
            .map(|p| root.join(p))
            .collect()
    }

    pub fn excluded_paths(&self) -> Vec<&str> {
        self.config
            .excluded_paths
            .iter()
            .map(String::as_str)
            .collect()
    }
}
