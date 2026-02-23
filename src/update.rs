use color_eyre::owo_colors::OwoColorize;
use iced::Task;

use crate::{app_state::AppState, message::Message};

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
    }
}
