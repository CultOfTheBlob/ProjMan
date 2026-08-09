use std::sync::Arc;

use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::{
    app_state::AppState,
    project::{self, Existant, Project, valid_project::ValidProject},
};

static ABOUT: &str = "ProjMan";

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    List,
    Info {
        name: String,

        #[arg(long, short, default_value_t = false)]
        yaml: bool,
    },
    Path {
        name: String,
    },
    Template {
        name: String,
    },
    Repo {
        name: String,
    },
    License {
        name: String,
    },
    Open {
        name: String,
    },
}

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = ABOUT
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn parse_args(&self) {
        let app_state = match AppState::new() {
            Ok(app_state) => app_state,
            Err(err) => {
                eprintln!("{} {}", "[ERROR]:".red().bold(), err.to_string().bold());

                return;
            }
        };
        let app_state = Arc::new(app_state);

        let projects = project::load_projects()
            .unwrap_or_else(|err| {
                eprintln!("{} {}", "[ERROR]:".red().bold(), err.to_string().bold());

                vec![]
            })
            .into_iter()
            .filter_map(|project| {
                if let ValidProject::Existant(project) = project {
                    return Some(project);
                }

                None
            })
            .collect::<Vec<Arc<Project<Existant>>>>();

        let get_project =
            |name: &str| projects.iter().find(|&project| project.name == name);

        match &self.command {
            Some(Commands::List) => {
                for project in &projects {
                    println!("{}", project.name);
                }
            }
            Some(Commands::Info { name, yaml }) => {
                let Some(project) = get_project(name) else {
                    return;
                };

                match project.info(&app_state) {
                    Ok(info) if *yaml => {
                        if let Ok(info_yaml) = serde_yaml::to_string(&info) {
                            println!("{info_yaml}");
                        }
                    }

                    Ok(info) => println!("{info}"),

                    Err(err) => eprintln!(
                        "{} {}",
                        "[ERROR]:".red().bold(),
                        err.to_string().bold()
                    ),
                }
            }
            Some(Commands::Path { name }) => {
                let Some(project) = get_project(name) else {
                    eprintln!(
                        "{} {}",
                        "[ERROR]:".red().bold(),
                        format!("Could not find project {name}").bold()
                    );

                    return;
                };

                println!("{}", project.path.display());
            }
            Some(Commands::Template { name }) => {
                let Some(project) = get_project(name) else {
                    eprintln!(
                        "{} {}",
                        "[ERROR]:".red().bold(),
                        format!("Could not find project {name}").bold()
                    );

                    return;
                };

                println!("{}", project.template_name);
            }
            Some(Commands::Repo { name }) => {
                if let Some(project) = get_project(name) {
                    eprintln!(
                        "{} {}",
                        "[ERROR]:".red().bold(),
                        format!("Could not find project {name}").bold()
                    );

                    println!("{}", project.repo);
                }
            }
            Some(Commands::License { name }) => {
                let Some(project) = get_project(name) else {
                    eprintln!(
                        "{} {}",
                        "[ERROR]:".red().bold(),
                        format!("Could not find project {name}").bold()
                    );

                    return;
                };

                println!("{}", project.license);
            }
            Some(Commands::Open { name }) => {
                let Some(project) = get_project(name) else {
                    eprintln!(
                        "{} {}",
                        "[ERROR]:".red().bold(),
                        format!("Could not find project {name}").bold()
                    );

                    return;
                };

                if let Err(err) = project.run(&app_state) {
                    eprintln!("{}", err.to_string().red());
                }
            }
            None => (),
        }
    }
}
