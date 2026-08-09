use crate::{app_state::AppState, prelude::*, template::Template};
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, path::PathBuf};

pub use load_projects::load_projects;

mod create;
mod existant;
pub mod info;
mod load_projects;
mod nonexistant;
mod unvalidated;
pub mod valid_project;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project<State = Unvalidated> {
    pub name: String,
    pub path: PathBuf,
    pub repo: String,
    pub license: String,
    pub template_name: String,

    #[serde(skip)]
    pub state: PhantomData<State>,
}

#[derive(Debug, Default, Clone)]
pub struct Existant;

#[derive(Debug, Default, Clone)]
pub struct Nonexistant;

#[derive(Debug, Default, Clone)]
pub struct Unvalidated;

impl<State: Default> Project<State> {
    pub const PROJECT_FILE_NAME: &str = "projman.toml";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(self, name: String) -> Self {
        Self { name, ..self }
    }

    pub fn path(self, path: PathBuf) -> Self {
        Self { path, ..self }
    }

    pub fn repo(self, repo: String) -> Self {
        Self { repo, ..self }
    }

    pub fn template_name(self, template_name: String) -> Self {
        Self {
            template_name,
            ..self
        }
    }

    pub fn get_project_file_path(&self) -> PathBuf {
        PathBuf::from(&self.path).join(Project::<()>::PROJECT_FILE_NAME)
    }

    pub fn get_template<'a>(&self, app_state: &'a AppState) -> Result<&'a Template> {
        app_state.get_template(&self.template_name)
    }
}
