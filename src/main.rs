use color_eyre::owo_colors::OwoColorize;
use iced::{Theme, application};

mod app_state;
mod boot;
mod message;
mod update;
mod view;

mod config;

use crate::{boot::boot, config::Config, update::update, view::view};

fn main() -> iced::Result
{
    let config = match Config::read_config_file()
    {
        Ok(config) => config,
        Err(err) =>
        {
            println!("{}", err.yellow());
            return Ok(());
        }
    };

    if let Err(err) = config.is_valid()
    {
        println!("{:?}", err.red());
        return Ok(());
    }

    application(boot, update, view)
        .title("ProjMan")
        .theme(Theme::Nord)
        .run()
}
