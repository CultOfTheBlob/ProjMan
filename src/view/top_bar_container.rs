use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row},
};

use crate::{app_state::AppState, message::Message};

pub fn build<'a>(state: &AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    let mut top_bar_content = row![
        button("Create")
            .style(button::secondary)
            .on_press(Message::Create),
        button("Import")
            .style(button::secondary)
            .on_press(Message::Import)
    ]
    .spacing(12);

    if state.selected_project.is_some()
    {
        top_bar_content = top_bar_content.push(
            button("Remove")
                .style(button::primary)
                .on_press(Message::Remove),
        );
    }

    let top_bar = container(top_bar_content)
        .width(Length::Fill)
        .padding(8)
        .style(|theme: &Theme| container::Style {
            background: Some(Color(theme.extended_palette().background.weak.color)),
            border: Border {
                color: theme.extended_palette().background.strongest.color,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        });

    column![top_bar, content].into()
}
