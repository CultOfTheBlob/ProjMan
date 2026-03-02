mod boot;
mod message;
mod state;
mod update;
mod view;

use color_eyre::owo_colors::OwoColorize;
use iced::application;

use crate::{boot::boot, state::config::Config, update::update, view::view};

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
        .theme(config.theme.theme.convert_to_iced_theme())
        .run()
}
