use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row, space, text},
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

    let sidebar_expand_widget =
        button(text(if state.sidebar_expanded { "" } else { "" }).size(24))
            .style(button::subtle)
            .on_press_maybe(
                if state.pending.is_none()
                {
                    Some(Message::SideBarToggled)
                }
                else
                {
                    None
                },
            );

    let top_bar_content = row![
        create_widget,
        import_widget,
        space().width(Length::Fill),
        sidebar_expand_widget
    ]
    .spacing(12);

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
