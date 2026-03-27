use std::sync::Arc;

use iced::Task;

use crate::{
    message::Message,
    state::{
        app_state::{AppState, NotifKind},
        config::Config,
    },
    templates::Templates,
};

pub fn boot() -> (AppState, Task<Message>)
{
    let config: Config = match Config::read_config_file()
    {
        Ok(config) => config,
        Err(err) =>
        {
            let mut app_state: AppState = AppState::default();
            app_state.push_notification(err.get_message(), NotifKind::Warning);

            return (app_state, Task::none());
        }
    };

    let templates: Templates = match Templates::generate()
    {
        Ok(templates) => templates,
        Err(err) =>
        {
            let mut app_state: AppState = AppState::default();
            app_state.push_notification(err.get_message(), NotifKind::Warning);

            return (app_state, Task::none());
        }
    };

    let mut state: AppState = AppState::default().templates(templates).config(config);

    match state.load_projects()
    {
        Ok(projects) => state.project_list = Arc::new(projects),
        Err(err) => state.push_notification(err.get_message(), NotifKind::Error),
    };

    (state, Task::none())
}
