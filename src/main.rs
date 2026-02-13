use std::{
    fs::{File, create_dir_all, read_to_string},
    io::{self},
    path::PathBuf,
};

use color_eyre::owo_colors::OwoColorize;
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
mod config;
mod input_handler;
mod ui;

use crate::{
    app::{App, CurrentScreen, Project},
    config::Config,
    input_handler::{Loop, handle_input},
    ui::{create, main},
};

fn main() -> color_eyre::Result<()>
{
    color_eyre::install()?;

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

    let project_list: Vec<Project> = match ProjectDirs::from("", "", "projman")
    {
        Some(proj_dirs) =>
        {
            create_dir_all(proj_dirs.data_dir())?;

            let data_path: PathBuf = proj_dirs.data_dir().join("projects.json");

            if data_path.is_file()
            {
                let proj: Vec<Project> = serde_json::from_str(&read_to_string(&data_path)?)?;

                proj
            }
            else
            {
                File::create(&data_path)?;

                Vec::<Project>::new()
            }
        }
        _ => Vec::<Project>::new(),
    };

    enable_raw_mode()?;

    let mut stderr: io::Stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend: CrosstermBackend<io::Stderr> = CrosstermBackend::new(stderr);
    let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;

    let mut app: App = App::new().projects(project_list);
    let res: io::Result<()> = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res
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
        match app.current_screen
        {
            CurrentScreen::Main =>
            {
                terminal.draw(|f| main::ui(f, app))?;
            }

            CurrentScreen::Create =>
            {
                terminal.draw(|f| create::ui(f, app))?;
            }
        }

        match handle_input(app)?
        {
            Some(Loop::Continue) => continue,
            Some(Loop::Break) => break Ok(()),
            None => (),
        }
    }
}
