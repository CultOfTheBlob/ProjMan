use iced::Task;

use crate::{
    message::Message,
    state::{
        app_state::{AppState, NotifKind},
        config::Config,
    },
};

pub fn boot() -> (AppState, Task<Message>)
{
    let config = match Config::read_config_file()
    {
        Ok(config) => config,
        Err(err) =>
        {
            let mut app_state: AppState = AppState::default();
            app_state.push_notification(err.get_message(), NotifKind::Warning);

            return (app_state, Task::none());
        }
    };

    (
        AppState::default().with_config(config).apply_config(),
        Task::none(),
    )
}
