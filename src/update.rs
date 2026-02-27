use color_eyre::owo_colors::OwoColorize;
use iced::Task;

use crate::{
    app_state::{AppState, Popup},
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

            match state.create_project()
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
