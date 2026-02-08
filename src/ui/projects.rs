use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut list_items: Vec<ListItem> = Vec::<ListItem>::new();

    for project in &app.project_list
    {
        let item: String = format!("{: <20} : {:?}", project.name, project.project_type);

        list_items.push(ListItem::new(Line::from(Span::styled(
            item,
            Style::default(),
        ))));
    }

    let list = List::new(list_items).block(list_block);

    frame.render_widget(list, chunks[1]);

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
