use std::path::PathBuf;

use iced::{
    Background::Color,
    Border, Element, Length, Renderer, Theme,
    widget::{Container, button, column, combo_box, container, row, stack, text, text_input},
};

use crate::{
    app_state::{AppState, Popup},
    message::Message,
};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    if !matches!(state.pending, Some(Popup::Create))
    {
        return content;
    }

    let popup_content: Container<'_, Message, Theme, Renderer> = container(
        column![
            container(text("Create Project"))
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
                }),
            container(
                text_input("Name...", &state.new_project.name)
                    .on_input(Message::ChangeNewProjectName)
            )
            .padding(12),
            container(combo_box(
                &state.project_types,
                "Select project type...",
                Some(&state.new_project.project_type),
                Message::ChangeNewProjectType
            ))
            .padding(12),
            container(
                text_input("Path...", &state.new_project.path.to_string_lossy())
                    .on_input(|path| Message::ChangeNewProjectPath(PathBuf::from(path)))
            )
            .padding(12),
            row![
                button("Cancel")
                    .style(button::secondary)
                    .on_press(Message::CancelCreate),
                button("Confirm")
                    .style(button::primary)
                    .on_press(Message::ConfirmCreate)
            ]
            .padding(12)
            .spacing(256)
        ]
        .spacing(12),
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
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ]
    .into()
}
