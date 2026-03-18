use color_eyre::owo_colors::OwoColorize;
use iced::Task;

use crate::{
    message::Message,
    state::{app_state::AppState, config::Config},
};

pub fn boot() -> (AppState, Task<Message>)
{
    let config = match Config::read_config_file()
    {
        Ok(config) => config,
        Err(err) =>
        {
            println!("{}", err.get_message().yellow());
            return (AppState::default(), Task::none());
        }
    };

    (
        AppState::default().with_config(config).apply_config(),
        Task::none(),
    )
}
