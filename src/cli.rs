use std::sync::Arc;

use clap::{Args, Parser, ValueEnum};
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
    #[command(flatten)]
    project: ProjectArgs,

    #[arg(short = 'L', long)]
    list: bool,
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

        if self.list
        {
            for project in state.project_list.iter()
            {
                println!("{}", project.name);
            }
        }

        if let Some(name) = &self.project.name
        {
            let project = match state.get_project(name)
            {
                Ok(project) => project,
                Err(err) =>
                {
                    eprintln!("{}", err.get_message().red());
                    return;
                }
            };

            match self.project.option
            {
                Some(ProjectOption::Info) =>
                {
                    println!("{:#?}", project.info());
                }
                Some(ProjectOption::Path) =>
                {
                    println!("{}", project.path.display());
                }
                Some(ProjectOption::Template) =>
                {
                    println!("{}", project.template_name);
                }
                Some(ProjectOption::Repo) =>
                {
                    println!("{}", project.repo);
                }
                Some(ProjectOption::License) =>
                {
                    println!("{}", project.license);
                }
                Some(ProjectOption::Open) =>
                {
                    if let Err(err) = project.run()
                    {
                        eprintln!("{}", err.get_message().red());
                    }
                }

                None => (),
            }
        }
    }
}

#[derive(Debug, Args)]
struct ProjectArgs
{
    #[arg(long = "project", requires = "option")]
    name: Option<String>,

    #[arg(requires = "name")]
    option: Option<ProjectOption>,
}

#[derive(Debug, Clone, ValueEnum)]
enum ProjectOption
{
    Info,
    Path,
    Template,
    Repo,
    License,
    Open,
}
