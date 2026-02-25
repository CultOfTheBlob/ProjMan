use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;
use iced::Task;

use crate::{
    app_state::{AppState, Project},
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
            state.selected_project = Some(index);

            Task::none()
        }
        Message::Remove(index) =>
        {
            match state.remove_project(index, false)
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            }

            state.selected_project = None;

            Task::none()
        }

        Message::Create =>
        {
            match state.create_project(Project {
                name: String::from("TestProject4"),
                path: PathBuf::from("/home/blob/Projects/TestProject4/"),
                project_type: crate::app_state::ProjectType::Test,
            })
            {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err.red()),
            }

            Task::none()
        }

        Message::Import => Task::none(),
    }
}
