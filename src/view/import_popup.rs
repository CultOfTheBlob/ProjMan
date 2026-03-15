use std::path::PathBuf;

use iced::{
    Alignment,
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row, stack, text, text_input},
};

use crate::{
    message::Message,
    state::{
        app_state::{AppState, Popup},
        project::Project,
    },
};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    if !matches!(state.pending, Some(Popup::Import))
    {
        return content;
    }

    let project_path_widget = container(
        column![
            row![
                text(&state.config.general.projects_dir).style(text::secondary),
                text_input("...", &state.import_project_path,)
                    .on_input(Message::ImportProjectPathChanged)
            ]
            .spacing(4)
            .align_y(Alignment::Center),
            if !Project::is_project_path(
                PathBuf::from(&state.config.general.projects_dir).join(&state.import_project_path)
            )
            {
                container(text("This path is not a valid project!").style(text::danger))
            }
            else
            {
                container(text(""))
            }
        ]
        .spacing(8),
    )
    .padding(12);

    let project_name_widget = container(
        row![
            text("Project Name:"),
            text_input("", &state.import_project_name).on_input(Message::ImportProjectNameChanged)
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .padding(12);

    let cancel_widget = button("Cancel")
        .style(button::secondary)
        .on_press(Message::ImportCanceled);

    let confirm_widget = button("Confirm")
        .style(|theme: &Theme, status: button::Status| match status
        {
            button::Status::Disabled => button::Style {
                background: Some(Color(theme.extended_palette().background.weak.color)),
                ..button::primary(theme, status)
            },
            _ => button::primary(theme, status),
        })
        .on_press_maybe(
            if Project::is_project_path(
                PathBuf::from(&state.config.general.projects_dir).join(&state.import_project_path),
            )
            {
                Some(Message::ImportConfirmed)
            }
            else
            {
                None
            },
        );

    let popup_content = container(
        column![
            project_path_widget,
            project_name_widget,
            row![cancel_widget, confirm_widget].spacing(256)
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
