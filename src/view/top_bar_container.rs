use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row},
};

use crate::{message::Message, state::app_state::AppState};

pub fn build<'a>(state: &AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    let create_widget = button("Create").style(button::secondary).on_press_maybe(
        if state.pending.is_none()
        {
            Some(Message::Created)
        }
        else
        {
            None
        },
    );

    let import_widget = button("Import").style(button::secondary).on_press_maybe(
        if state.pending.is_none()
        {
            Some(Message::Imported)
        }
        else
        {
            None
        },
    );

    let top_bar_content = row![create_widget, import_widget].spacing(12);

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
