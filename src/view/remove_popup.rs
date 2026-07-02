use crate::{
    message::Message,
    state::app_state::{AppState, Popup},
};
use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, checkbox, column, container, row, stack, text},
};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message> {
    if !matches!(state.pending, Some(Popup::Remove)) {
        return content;
    }

    let remove_warning_widget = container(text("Are you sure you want to remove this project?"));

    let delete_project_folder_widget = container(
        checkbox(state.delete_project_folder)
            .style(checkbox::danger)
            .label("Also remove project folder")
            .on_toggle(Message::RemoveProjectFolderToggled),
    );

    let cancel_widget = button("Cancel")
        .style(button::secondary)
        .on_press(Message::RemoveCanceled);

    let confirm_widget = button("Confirm")
        .style(|theme: &Theme, status| {
            if state.delete_project_folder {
                button::danger(theme, status)
            } else {
                button::warning(theme, status)
            }
        })
        .on_press(Message::RemoveConfirmed);

    let popup_content = container(column![
        remove_warning_widget.padding(16),
        delete_project_folder_widget.padding(16),
        row![cancel_widget, confirm_widget].padding(16).spacing(256)
    ])
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
