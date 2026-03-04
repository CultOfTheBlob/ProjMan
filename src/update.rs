use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;
use iced::{Task, futures::TryFutureExt};

use crate::{
    message::Message,
    state::{
        app_state::{AppState, Popup},
        project::Project,
    },
};

pub fn update(state: &mut AppState, message: Message) -> Task<Message>
{
    match message
    {
        Message::Open(project) =>
        {
            match project.run()
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            };

            Task::none()
        }
        Message::Selected(index) =>
        {
            state.selected_project = Some(index);

            Task::none()
        }
        Message::Remove =>
        {
            state.pending = Some(Popup::Remove);

            Task::none()
        }
        Message::ConfirmRemove =>
        {
            if let Some(Popup::Remove) = state.pending
            {
                match state.remove_project()
                {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", err.red()),
                }
            }

            state.selected_project = None;
            state.pending = None;
            state.delete_project_folder = state.config.general.delete_project_folder;

            Task::none()
        }
        Message::CancelRemove =>
        {
            state.pending = None;
            state.delete_project_folder = state.config.general.delete_project_folder;

            Task::none()
        }
        Message::ToggleRemoveProjectFolder(delete_project_folder) =>
        {
            state.delete_project_folder = delete_project_folder;

            Task::none()
        }
        Message::Create =>
        {
            state.pending = Some(Popup::Create);

            Task::none()
        }
        Message::ConfirmCreate =>
        {
            if !state
                .new_project
                .path_is_valid(&state.config.general.projects_dir)
                .0
            {
                return Task::none();
            }

            state.project_creation_status = (true, String::from("Creating project..."));
            Task::perform(
                AppState::create_project(state.new_project.clone()).map_err(|e| e.to_string()),
                Message::FinishCreate,
            )
        }
        Message::FinishCreate(create_result) =>
        {
            match create_result
            {
                Ok(projects_list) =>
                {
                    state.project_creation_status = (false, String::new());
                    state.project_list = projects_list;
                    state.new_project = Project::default(&state.config);
                    state.selected_project = Some(state.project_list.len() - 1);
                    state.pending = None;
                    state.new_project_path_changed = false;
                }
                Err(err) =>
                {
                    state.project_creation_status.0 = false;
                    state.project_creation_status.1 = format!("Error: {err}");
                }
            }

            Task::none()
        }
        Message::CancelCreate =>
        {
            state.project_creation_status = (false, String::new());
            state.new_project = Project::default(&state.config);
            state.pending = None;
            state.new_project_path_changed = false;

            Task::none()
        }
        Message::ChangeNewProjectName(name) =>
        {
            if !state.new_project_path_changed
            {
                state.new_project.path =
                    PathBuf::from(&state.config.general.projects_dir).join(&name);
            }

            state.new_project.name = name;

            Task::none()
        }
        Message::ChangeNewProjectType(project_type) =>
        {
            state.new_project.project_type = project_type;

            Task::none()
        }
        Message::ChangeNewProjectRepo(repo) =>
        {
            state.new_project.repo = repo;

            Task::none()
        }
        Message::ChangeNewProjectPath(path) =>
        {
            state.new_project.path = path;

            state.new_project_path_changed = true;

            Task::none()
        }
        Message::Import => Task::none(),
        Message::RemoveNonexistant =>
        {
            match state.remove_project()
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            }

            state.selected_project = None;
            Task::none()
        }
        Message::RestoreNonexistant =>
        {
            state.project_restoration_failed = false;
            state.restoring_project = true;

            Task::perform(
                AppState::restore_project(state.selected_project, state.project_list.clone())
                    .map_err(|e| e.to_string()),
                Message::FinishRemoveNonexistant,
            )
        }
        Message::FinishRemoveNonexistant(restore_result) =>
        {
            state.restoring_project = false;

            match restore_result
            {
                Ok(index) => state.project_list[index].exists = true,
                Err(_) => state.project_restoration_failed = true,
            }

            Task::none()
        }
    }
}
