#![allow(clippy::single_match)]

use std::io;

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

use crate::{app::App, ui::projects::ui};

fn main() -> color_eyre::Result<()>
{
    enable_raw_mode()?;

    let mut stderr: io::Stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend: CrosstermBackend<io::Stderr> = CrosstermBackend::new(stderr);
    let mut terminal: Terminal<CrosstermBackend<io::Stderr>> = Terminal::new(backend)?;

    let mut app: App = App::new();
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
