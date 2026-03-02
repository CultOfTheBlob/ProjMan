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
                column![
                    text("Project Name:"),
                    text_input("", &state.new_project.name).on_input(Message::ChangeNewProjectName)
                ]
                .spacing(4)
            )
            .padding(12),
            container(
                column![
                    text("Project Type:"),
                    combo_box(
                        &state.project_types,
                        "",
                        Some(&state.new_project.project_type),
                        Message::ChangeNewProjectType
                    )
                ]
                .spacing(4)
            )
            .padding(12),
            container(
                column![
                    text("Project Path:"),
                    column![
                        text_input("", &state.new_project.path.to_string_lossy())
                            .on_input(|path| Message::ChangeNewProjectPath(PathBuf::from(path)))
                            .style(|theme: &Theme, status| {
                                let mut style = text_input::default(theme, status);

                                if !state
                                    .new_project
                                    .path_is_valid(&state.config.general.projects_dir)
                                    .0
                                {
                                    style.border.color = theme.extended_palette().danger.base.color
                                }

                                style
                            }),
                        text(
                            state
                                .new_project
                                .path_is_valid(&state.config.general.projects_dir)
                                .1
                        )
                        .height(
                            if state
                                .new_project
                                .path_is_valid(&state.config.general.projects_dir)
                                .0
                            {
                                0.into()
                            }
                            else
                            {
                                Length::Shrink
                            }
                        )
                        .style(|theme: &Theme| text::danger(theme))
                    ]
                    .spacing(2)
                ]
                .spacing(4)
            )
            .padding(12),
            row![
                button("Cancel")
                    .style(button::secondary)
                    .on_press(Message::CancelCreate),
                button("Confirm")
                    .style(|theme: &Theme, status| {
                        let mut style = button::primary(theme, status);

                        if !state
                            .new_project
                            .path_is_valid(&state.config.general.projects_dir)
                            .0
                        {
                            style.background =
                                Some(Color(theme.extended_palette().background.weak.color))
                        }

                        style
                    })
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
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ]
    .into()
}
