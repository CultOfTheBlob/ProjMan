use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;
use iced::Task;

use crate::{
    app_state::{AppState, Popup, Project},
    message::Message,
};

pub fn update(state: &mut AppState, message: Message) -> Task<Message>
{
    match message
    {
        Message::Open(project) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            match project.run()
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            };

            Task::none()
        }
        Message::Selected(index) =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            state.selected_project = Some(index);

            Task::none()
        }
        Message::Remove =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

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
            if state.pending.is_some()
            {
                return Task::none();
            }

            state.pending = Some(Popup::Create);

            Task::none()
        }
        Message::ConfirmCreate =>
        {
            if !state.new_project.path_is_valid().0
            {
                return Task::none();
            }

            match state.create_project()
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            }

            state.new_project = Project::default(&state.config);
            state.selected_project = Some(state.project_list.len() + 1);
            state.pending = None;
            state.new_project_path_changed = false;

            Task::none()
        }
        Message::CancelCreate =>
        {
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
        Message::ChangeNewProjectPath(path) =>
        {
            state.new_project.path = path;

            state.new_project_path_changed = true;

            Task::none()
        }
        Message::ChangeNewProjectType(project_type) =>
        {
            state.new_project.project_type = project_type;

            Task::none()
        }

        Message::Import =>
        {
            if state.pending.is_some()
            {
                return Task::none();
            }

            Task::none()
        }
    }
}
