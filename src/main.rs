//! ``ProjMan``

use crate::{cli::Cli, config::Config, log::Log};
use clap::Parser as _;

mod app_state;
mod cli;
mod config;
mod config_dir;
mod error;
mod log;
mod prelude;
mod project;
mod root_view;
mod template;
mod theme;
mod utils;

fn main() {
    let config = Config::load().unwrap_or_else(|err| {
        Log::Error.log(&err.to_string());

        Config::default()
    });

    let cli = Cli::parse();

    if cli.command.is_some() {
        cli.parse_args();
        return;
    }

    utils::run_app(config);
}
