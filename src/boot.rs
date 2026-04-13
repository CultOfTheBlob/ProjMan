use crate::{
    message::Message,
    state::{
        app_state::{AppState, NotifKind},
        config::Config,
    },
    templates::Templates,
};
use iced::Task;
use std::sync::Arc;

pub fn boot() -> (AppState, Task<Message>)
{
    let config = match Config::read_config_file()
    {
        Ok(config) => config,
        Err(err) =>
        {
            let mut app_state = AppState::default();
            app_state.push_notification(err.get_message(), NotifKind::Warning);

            return (app_state, Task::none());
        }
    };

    let templates = match Templates::generate()
    {
        Ok(templates) => templates,
        Err(err) =>
        {
            let mut app_state = AppState::default();
            app_state.push_notification(err.get_message(), NotifKind::Warning);

            return (app_state, Task::none());
        }
    };

    let mut state = AppState::default().templates(templates).config(config);

    match state.load_projects()
    {
        Ok(projects) => state.project_list = Arc::new(projects),
        Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
    }

    (state, Task::none())
}
