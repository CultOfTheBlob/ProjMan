use crate::{
    log::Log,
    prelude::*,
    project::{self, Existant, Project, valid_project::ValidProject},
    template::{self, Template},
};
use gpui::{App, BorrowAppContext as _, Global};
use std::{
    collections::BTreeMap,
    fs::{self},
    path::{Path, PathBuf},
    sync::Arc,
};
use update_projects_file::update_projects_file;

mod update_projects_file;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: BTreeMap<String, Template>,
    pub projects: Vec<ValidProject>,

    pub selected_project_index: Option<usize>,
    pub restoring_project: bool,
    pub creating_project: bool,
    pub modal_active: bool,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let templates = template::load_templates()?;

        let projects = project::load_projects()?;

        Ok(Self {
            templates,
            projects,

            selected_project_index: None,
            restoring_project: false,
            creating_project: false,
            modal_active: false,
        })
    }

    pub fn set_modal_active(cx: &mut App, modal_active: bool) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.modal_active = modal_active;
        });
    }

    pub fn set_selected_project_index(
        cx: &mut App,
        selected_project_index: Option<usize>,
    ) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.selected_project_index = selected_project_index;
        });
    }

    pub fn set_restoring_project(cx: &mut App, restoring_project: bool) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.restoring_project = restoring_project;
        });
    }

    pub fn set_creating_project(cx: &mut App, creating_project: bool) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.creating_project = creating_project;
        });
    }

    pub fn get_template(&self, template_name: &str) -> Result<&Template> {
        self.templates
            .get(template_name)
            .ok_or_else(|| Error::GetTemplate(template_name.to_string()))
    }

    pub fn get_filtered_projects(&self, filter: &str) -> Vec<&ValidProject> {
        let filter = filter.to_lowercase();

        self.projects
            .iter()
            .filter(|project| {
                let name = match &project {
                    ValidProject::Existant(p) => &p.name,
                    ValidProject::Nonexistant(p) => &p.name,
                };

                name.to_lowercase().contains(&filter)
            })
            .collect()
    }

    pub fn get_selected_project(&self) -> Option<ValidProject> {
        self.selected_project_index
            .map(|index| self.projects[index].clone())
    }

    pub fn run_project(&self, project: &Arc<Project<Existant>>) -> Result<()> {
        project.run(self)?;

        Ok(())
    }

    pub fn add_project(&mut self, project: Project<Existant>) -> Result<()> {
        self.projects
            .push(ValidProject::Existant(Arc::new(project)));

        update_projects_file(&self.projects)?;

        Ok(())
    }

    pub fn remove_project(
        &mut self,
        project_index: usize,
        remove_project_folder: bool,
    ) -> Result<()> {
        let project = self.projects[project_index].clone();

        self.projects.remove(project_index);

        self.selected_project_index = None;

        if let ValidProject::Existant(project) = project {
            let project_path = PathBuf::from(&project.path);

            let project_file_path = project.get_project_file_path();

            fs::remove_file(project_file_path)
                .map_err(|err| Error::RemoveProject(err.to_string()))?;

            if remove_project_folder {
                fs::remove_dir_all(project_path)
                    .map_err(|err| Error::RemoveProject(err.to_string()))?;
            }
        }

        update_projects_file(&self.projects)?;

        Ok(())
    }

    pub fn restore_project(&mut self, project: Project<Existant>) -> Result<()> {
        let Some(selected_project) = self.selected_project_index else {
            return Ok(());
        };

        self.projects[selected_project] = ValidProject::Existant(Arc::new(project));

        update_projects_file(&self.projects)?;

        Ok(())
    }

    pub fn edit_project(
        &mut self,
        project_index: usize,
        name: String,
        repo: String,
    ) -> Result<()> {
        let mut project = self.projects[project_index].clone();

        if let ValidProject::Existant(ref mut project_arc) = project {
            let project = Arc::make_mut(project_arc);

            if project.name == name && project.repo == repo {
                return Ok(());
            }

            project.name = name;
            project.repo = repo;

            let project_file_path = project.get_project_file_path();

            let project_file_contents = toml::to_string_pretty(&project)
                .map_err(|err| Error::EditProjects(err.to_string()))?;

            fs::write(&project_file_path, project_file_contents)
                .map_err(|err| Error::EditProjects(err.to_string()))?;
        }

        self.projects[project_index] = project;

        update_projects_file(&self.projects)?;

        Ok(())
    }

    pub fn import_project(&mut self, path: &Path) -> Result<()> {
        let project_already_exists = self.projects.iter().any(|project| {
            if let ValidProject::Existant(project) = project {
                return project.get_project_file_path() == path;
            }

            false
        });

        if project_already_exists {
            Log::Info.log("This project already exists!");

            return Ok(());
        }

        let project_file_contents = fs::read_to_string(path)
            .map_err(|err| Error::ImportProjects(err.to_string()))?;

        let project = toml::from_str::<Project<Existant>>(&project_file_contents)
            .map_err(|err| Error::ImportProjects(err.to_string()))?;

        self.projects
            .push(ValidProject::Existant(Arc::new(project)));

        update_projects_file(&self.projects)?;

        Ok(())
    }
}

impl Global for AppState {}

#[derive(Debug, Clone)]
pub struct GlobalAppState(pub Arc<AppState>);

impl Global for GlobalAppState {}
