use std::{
    collections::HashMap,
    fs::{File, create_dir_all, read_to_string},
    io::{self, Write},
    path::PathBuf,
};

use crossterm::style::Stylize;
use directories::ProjectDirs;
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod input_handler;
mod ui;

use crate::{
    app::{App, Project, ProjectType},
    input_handler::{Loop, handle_input},
    ui::projects::ui,
};

fn main() -> color_eyre::Result<()>
{
    color_eyre::install()?;

    let config: HashMap<String, String> = read_config_file()?;
    for option in config.keys()
    {
        match &option.as_str()
        {
            &"projects_dir" =>
            {
                let projects_dir: String = String::from(config.get(option).unwrap());

                if projects_dir.is_empty()
                    || !PathBuf::from(&projects_dir).is_dir()
                    || !projects_dir.ends_with('/')
                {
                    println!(
                        "{}",
                        "Error: Failed to run projman please make sure projects_dir \
                        in .config/projman/config.toml is a valid directory \
                        (Dont forget the trailing slash!!)"
                            .red()
                    );

                    return Ok(());
                }
            }

            _ => println!(
                "{}",
                format!("Warning: {option} is not a valid config option").yellow()
            ),
        }
    }

    enable_raw_mode()?;

    let mut stderr: io::Stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend: CrosstermBackend<io::Stderr> = CrosstermBackend::new(stderr);
    let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;

    //---TMP
    let project_list: Vec<Project> = vec![
        Project {
            name: String::from("TestProject1"),
            path: std::path::PathBuf::from(format!(
                "{}TestProject1/",
                config.get("projects_dir").unwrap()
            )),
            project_type: ProjectType::Test,
        },
        Project {
            name: String::from("TestProject2"),
            path: std::path::PathBuf::from(format!(
                "{}TestProject2/",
                config.get("projects_dir").unwrap()
            )),
            project_type: ProjectType::Test,
        },
        Project {
            name: String::from("TestProject3"),
            path: std::path::PathBuf::from(format!(
                "{}TestProject3/",
                config.get("projects_dir").unwrap()
            )),
            project_type: ProjectType::Test,
        },
    ];
    //---TMP

    let mut app: App = App::new().projects(project_list);
    let res: io::Result<()> = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(()) = res
    {
    }
    else if let Err(err) = res
    {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    std::io::Error: From<<B as Backend>::Error>,
{
    loop
    {
        terminal.draw(|f| ui(f, app))?;

        match handle_input(app)?
        {
            Some(Loop::Continue) => continue,
            Some(Loop::Break) => break Ok(()),
            None => (),
        }
    }
}

fn read_config_file() -> io::Result<HashMap<String, String>>
{
    if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
    {
        create_dir_all(proj_dirs.config_dir())?;

        let config_path: PathBuf = proj_dirs.config_dir().join("config.toml");

        if config_path.is_file()
        {
            let config: HashMap<String, String> =
                toml::from_str(&read_to_string(config_path)?).map_err(io::Error::other)?;

            return Ok(config);
        }

        let mut config_file: File = File::create(config_path)?;

        let mut config: HashMap<String, String> = HashMap::new();
        config.insert(String::from("projects_dir"), String::from(""));

        let config_toml = toml::to_string_pretty(&config).map_err(io::Error::other)?;

        config_file.write_all(config_toml.as_bytes())?;

        return Ok(config);
    }

    Ok(HashMap::new())
}
