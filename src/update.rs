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

        Message::Remove(project_index) =>
        {
            state.pending = Some(Popup::Remove(project_index));

            Task::none()
        }
        Message::ConfirmRemove =>
        {
            if let Some(Popup::Remove(project_index)) = state.pending
            {
                match state.remove_project(project_index, state.delete_project_folder)
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
        Message::RemoveProjectFolder(delete_project_folder) =>
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

            match state.create_project(Project {
                name: format!("TestProject{}", state.project_list.len() + 1),
                path: PathBuf::from(format!(
                    "/home/blob/Projects/TestProject{}/",
                    state.project_list.len() + 1
                )),
                project_type: crate::app_state::ProjectType::Test,
            })
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            }

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
