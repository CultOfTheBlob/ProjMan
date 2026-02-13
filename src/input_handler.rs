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
                KeyCode::Char('q') => return Ok(Some(Loop::Break)),

                KeyCode::Char('a') => app.current_screen = CurrentScreen::Create,

                KeyCode::Up => app.decrement_current_project(),
                KeyCode::Char('k') => app.decrement_current_project(),

                KeyCode::Down => app.increment_current_project(),
                KeyCode::Char('j') => app.increment_current_project(),

                KeyCode::Enter => app.project_list[*app.get_current_project_index()].run()?,
                KeyCode::Char('l') => app.project_list[*app.get_current_project_index()].run()?,

                _ => (),
            },

            CurrentScreen::Create => match key.code
            {
                KeyCode::Esc => app.current_screen = CurrentScreen::Main,

                KeyCode::Char('l') => match app.current_tab
                {
                    Some(mut current_tab) =>
                    {
                        let tabs_len: usize = app.creation_menu_tabs.len();

                        if current_tab >= tabs_len - 1
                        {
                            current_tab = 0;
                        }
                        else
                        {
                            current_tab += 1;
                        }

                        app.current_tab = Some(current_tab)
                    }
                    None => app.current_tab = Some(0),
                },
                KeyCode::Char('h') => match app.current_tab
                {
                    Some(mut current_tab) =>
                    {
                        let tabs_len: usize = app.creation_menu_tabs.len();

                        if current_tab == 0
                        {
                            current_tab = tabs_len - 1;
                        }
                        else
                        {
                            current_tab -= 1;
                        }

                        app.current_tab = Some(current_tab)
                    }
                    None => app.current_tab = Some(0),
                },

                _ => (),
            },
        }
    }

    Ok(None)
}
