mod boot;
#[macro_use]
mod error;
mod cli;
mod message;
mod project;
mod state;
mod templates;
mod update;
mod view;

use crate::{cli::Cli, message::Message, state::config::Config};
use clap::Parser as _;
use color_eyre::owo_colors::OwoColorize as _;
use iced::Result as IcedResult;
use std::time::Duration;

fn main() -> IcedResult {
    let cli = Cli::parse();

    if cli.command.is_some() {
        cli.parse_args();
        return Ok(());
    }

    let config = match Config::read_config_file() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err.get_message().yellow());
            return Ok(());
        }
    };

    if let Err(err) = config.is_valid() {
        eprintln!("{:?}", err.red());
        return Ok(());
    }

    #[expect(clippy::large_include_file)]
    let font = include_bytes!("../fonts/JetBrainsMonoNerdFontMono-Regular.ttf");

    iced::application(boot::boot, update::update, view::view)
        .title("ProjMan")
        .theme(config.theme.theme.convert_to_iced_theme())
        .font(font.as_slice())
        .subscription(|_| iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick))
        .run()
}
