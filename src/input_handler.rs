use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::app::{App, CurrentScreen};

pub fn handle_input(app: &mut App) -> io::Result<bool>
{
    if let Event::Key(key) = event::read()?
    {
        if key.kind == event::KeyEventKind::Release
        {
            return Ok(true);
        }

        match app.current_screen
        {
            CurrentScreen::Main => match key.code
            {
                KeyCode::Char('q') =>
                {
                    return Ok(false);
                }

                _ => (),
            },
        }
    }

    Ok(false)
}
