use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::app::{App, CurrentScreen};

#[derive(Debug)]
pub enum Loop
{
    Continue,
    Break,
}

pub fn handle_input(app: &mut App) -> io::Result<Option<Loop>>
{
    if let Event::Key(key) = event::read()?
    {
        if key.kind == event::KeyEventKind::Release
        {
            return Ok(Some(Loop::Continue));
        }

        match app.current_screen
        {
            CurrentScreen::Main => match key.code
            {
                KeyCode::Char('q') =>
                {
                    return Ok(Some(Loop::Break));
                }

                _ => (),
            },
        }
    }

    Ok(None)
}
