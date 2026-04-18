use std::sync::Arc;

use clap::{Parser, Subcommand};
use color_eyre::owo_colors::OwoColorize as _;

use crate::{
    state::{app_state::AppState, config::Config},
    templates::Templates,
};

static ABOUT: &str = "ProjMan is a tool to manage different kinds of projects with nix";

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = ABOUT
)]
pub struct Cli
{
    #[command(subcommand)]
    pub command: Option<Commands>,
}
impl Cli
{
    pub fn parse_args(&self)
    {
        let config = match Config::read_config_file()
        {
            Ok(config) => config,
            Err(err) =>
            {
                eprintln!("{}", err.get_message().yellow());
                return;
            }
        };

        if let Err(err) = config.is_valid()
        {
            eprintln!("{:?}", err.red());
            return;
        }

        let templates = match Templates::generate()
        {
            Ok(templates) => templates,
            Err(err) =>
            {
                eprintln!("{}", err.get_message().red());
                return;
            }
        };

        let mut state = AppState::default().config(config).templates(templates);

        match state.load_projects()
        {
            Ok(projects) => state.project_list = Arc::new(projects),
            Err(err) =>
            {
                eprintln!("{}", err.get_message().red());
                return;
            }
        }

        let project = |name: &str| match state.get_project(name)
        {
            Ok(p) => Some(p),
            Err(err) =>
            {
                eprintln!("{}", err.get_message().red());
                None
            }
        };

        match &self.command
        {
            Some(Commands::List) =>
            {
                for p in state.project_list.iter()
                {
                    println!("{}", p.name);
                }
            }
            Some(Commands::Info { name }) =>
            {
                if let Some(p) = project(name)
                {
                    match p.info()
                    {
                        Some(info) => println!("{info}"),
                        None => println!(),
                    }
                }
            }
            Some(Commands::Path { name }) =>
            {
                if let Some(p) = project(name)
                {
                    println!("{}", p.path.display());
                }
            }
            Some(Commands::Template { name }) =>
            {
                if let Some(p) = project(name)
                {
                    println!("{}", p.template_name);
                }
            }
            Some(Commands::Repo { name }) =>
            {
                if let Some(p) = project(name)
                {
                    println!("{}", p.repo);
                }
            }
            Some(Commands::License { name }) =>
            {
                if let Some(p) = project(name)
                {
                    println!("{}", p.license);
                }
            }
            Some(Commands::Open { name }) =>
            {
                if let Some(p) = project(name)
                    && let Err(err) = p.run()
                {
                    eprintln!("{}", err.get_message().red());
                }
            }
            None => (),
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands
{
    List,
    Info
    {
        name: String,
    },
    Path
    {
        name: String,
    },
    Template
    {
        name: String,
    },
    Repo
    {
        name: String,
    },
    License
    {
        name: String,
    },
    Open
    {
        name: String,
    },
}
