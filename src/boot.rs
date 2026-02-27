use color_eyre::owo_colors::OwoColorize;
use iced::Task;

use crate::{app_state::AppState, config::Config, message::Message};

pub fn boot() -> (AppState, Task<Message>)
{
    let config = match Config::read_config_file()
    {
        Ok(config) => config,
        Err(err) =>
        {
            println!("{}", err.yellow());
            return (AppState::default(), Task::none());
        }
    };

    (
        AppState::default().with_config(config).apply_config(),
        Task::none(),
    )
}
