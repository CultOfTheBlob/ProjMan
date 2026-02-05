use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App)
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Projects", Style::default().fg(Color::Blue)))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    let view_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let list = Paragraph::new(Text::styled("", Style::default().fg(Color::Blue))).block(list_block);

    frame.render_widget(list, view_chunks[0]);

    let opts_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let opts = Paragraph::new(Text::styled("", Style::default().fg(Color::Blue))).block(opts_block);

    frame.render_widget(opts, view_chunks[1]);

    let current_keys_hint = {
        match app.current_screen
        {
            CurrentScreen::Main => Span::styled("(q) to quit", Style::default().fg(Color::Green)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(key_notes_footer, chunks[2]);
}
