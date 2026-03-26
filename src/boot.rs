use iced::Task;

use crate::{
    message::Message,
    state::{
        app_state::{AppState, NotifKind},
        config::Config,
    },
    templates::{Templates, template::Template},
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

    let templates: Vec<Template> = match Templates::default().generate()
    {
        Ok(templates) => templates.templates().to_vec(),
        Err(err) =>
        {
            let mut app_state: AppState = AppState::default();
            app_state.push_notification(err.get_message(), NotifKind::Warning);

            return (app_state, Task::none());
        }
    };

    (
        AppState::default().templates(templates).config(config),
        Task::none(),
    )
}
