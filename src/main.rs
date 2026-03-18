mod boot;
mod error;
mod message;
mod state;
mod templates;
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
            println!("{}", err.get_message().yellow());
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
        .font(include_bytes!("../fonts/JetBrainsMonoNerdFontMono-Regular.ttf").as_slice())
        .run()
}
