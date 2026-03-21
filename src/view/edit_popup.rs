use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row, stack, text, text_input},
};

use crate::{
    message::Message,
    state::app_state::{AppState, Popup},
};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    if !matches!(state.pending, Some(Popup::Edit))
    {
        return content;
    }

    let title_bar = container(text("Edit Project"))
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

    let project_name_widget = container(
        column![
            text("Project Name:"),
            text_input(&state.edit_project_name, &state.edit_project_name).on_input_maybe(
                if !state.project_creation_status.creating
                {
                    Some(Message::EditProjectNameChanged)
                }
                else
                {
                    None
                }
            )
        ]
        .spacing(4),
    );

    let project_repo_widget = container(
        column![
            text("Project Repo:"),
            text_input(&state.edit_project_repo, &state.edit_project_repo).on_input_maybe(
                if !state.project_creation_status.creating
                {
                    Some(Message::EditProjectRepoChanged)
                }
                else
                {
                    None
                }
            ),
        ]
        .spacing(4),
    );

    let cancel_widget = button("Cancel")
        .style(button::secondary)
        .on_press(Message::EditCanceled);

    let confirm_widget = button("Confirm")
        .style(button::primary)
        .on_press(Message::EditConfirmed);

    let popup_content = container(column![
        title_bar,
        project_name_widget.padding(16),
        project_repo_widget.padding(16),
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
