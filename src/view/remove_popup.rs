use iced::{
    Background::Color,
    Border, Element, Length, Renderer, Theme,
    widget::{Container, button, checkbox, column, container, row, stack, text},
};

use crate::{
    app_state::{AppState, Popup},
    message::Message,
};

pub fn build<'a>(state: &AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    if !matches!(state.pending, Some(Popup::Remove))
    {
        return content;
    }

    let popup_content: Container<'_, Message, Theme, Renderer> = container(
        column![
            text("Are you sure you want to remove this project?"),
            checkbox(state.delete_project_folder)
                .label("Also remove project folder")
                .on_toggle(Message::ToggleRemoveProjectFolder),
            row![
                button("Cancel")
                    .style(button::secondary)
                    .on_press(Message::CancelRemove),
                button("Confirm")
                    .style(button::primary)
                    .on_press(Message::ConfirmRemove)
            ]
            .spacing(256)
        ]
        .spacing(16)
        .padding(24),
    )
    .width(Length::Shrink)
    .style(|theme: &Theme| container::Style {
        background: Some(Color(theme.extended_palette().background.base.color)),
        border: Border {
            color: theme.extended_palette().background.strongest.color,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    });

    stack![
        content,
        container("")
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                })),
                ..Default::default()
            }),
        container(popup_content)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ]
    .into()
}
