use ratatui::{
    Frame,
    style::{Style, Stylize},
    symbols,
    widgets::{Block, Tabs},
};

use crate::app::App;
use crate::ui::{centered_rect, main};

pub fn ui(frame: &mut Frame, app: &App)
{
    main::ui(frame, app);

    let area = centered_rect(80, 60, frame.area());

    let tabs = Tabs::new(app.creation_menu_tabs.clone())
        .block(Block::bordered().title("Create:"))
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(app.current_tab)
        .divider(symbols::DOT)
        .padding(" <", "> ");

    frame.render_widget(tabs, area);
}
