mod boot;
#[macro_use]
mod error;
mod message;
mod project;
mod state;
mod templates;
mod update;
mod view;

use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use iced::application;

use crate::{boot::boot, message::Message, state::config::Config, update::update, view::view};

fn main() -> iced::Result
{
    let config = match Config::read_config_file()
    {
        Ok(config) => config,
        Err(err) =>
        {
            eprintln!("{}", err.get_message().yellow());
            return Ok(());
        }
    };

    if let Err(err) = config.is_valid()
    {
        eprintln!("{:?}", err.red());
        return Ok(());
    }

    application(boot, update, view)
        .title("ProjMan")
        .theme(config.theme.theme.convert_to_iced_theme())
        .font(include_bytes!("../fonts/JetBrainsMonoNerdFontMono-Regular.ttf").as_slice())
        .subscription(|_| iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick))
        .run()
}
