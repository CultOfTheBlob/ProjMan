use std::path::PathBuf;

use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, combo_box, container, row, stack, text, text_input},
};

use crate::{
    message::Message,
    state::app_state::{AppState, Popup},
};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    if !matches!(state.pending, Some(Popup::Create))
    {
        return content;
    }

    let title_bar = container(text("Create Project"))
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
            text_input("", &state.new_project.name).on_input_maybe(
                if !state.project_creation_status.0
                {
                    Some(Message::ChangeNewProjectName)
                }
                else
                {
                    None
                }
            )
        ]
        .spacing(4),
    )
    .padding(12);

    let project_type_widget = if !state.project_creation_status.0
    {
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
            .spacing(4),
        )
        .padding(12)
    }
    else
    {
        container(
            column![
                text("Project Type:"),
                text_input(&state.new_project.project_type.to_string(), "")
            ]
            .spacing(4),
        )
        .padding(12)
    };

    let project_repo_widget = container(
        column![
            text("Project Repo:"),
            text_input(
                "https://github.com/Author/Project.git",
                &state.new_project.repo
            )
            .on_input_maybe(
                if !state.project_creation_status.0
                {
                    Some(Message::ChangeNewProjectRepo)
                }
                else
                {
                    None
                }
            ),
        ]
        .spacing(4),
    )
    .padding(12);

    let project_path_widget = container(
        column![
            text("Project Path:"),
            column![
                text_input("", &state.new_project.path.to_string_lossy())
                    .on_input_maybe(
                        if !state.project_creation_status.0
                        {
                            Some(|path| Message::ChangeNewProjectPath(PathBuf::from(path)))
                        }
                        else
                        {
                            None
                        }
                    )
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
        .spacing(4),
    )
    .padding(12);

    let porgress_widget = container(text(&state.project_creation_status.1).style(
        if state.project_creation_status.1.starts_with("Error:")
        {
            text::danger
        }
        else
        {
            text::secondary
        },
    ))
    .width(Length::Fill)
    .padding(12);

    let cancel_widget = button("Cancel").style(button::secondary).on_press_maybe(
        if !state.project_creation_status.0
        {
            Some(Message::CancelCreate)
        }
        else
        {
            None
        },
    );

    let confirm_widget = button("Confirm")
        .style(|theme: &Theme, status| {
            let mut style = button::primary(theme, status);

            if !state
                .new_project
                .path_is_valid(&state.config.general.projects_dir)
                .0
            {
                style.background = Some(Color(theme.extended_palette().background.weak.color))
            }

            style
        })
        .on_press_maybe(
            if !state.project_creation_status.0
            {
                Some(Message::ConfirmCreate)
            }
            else
            {
                None
            },
        );

    let popup_content = container(
        column![
            title_bar,
            project_name_widget,
            project_type_widget,
            project_repo_widget,
            project_path_widget,
            porgress_widget,
            row![cancel_widget, confirm_widget].padding(12).spacing(256)
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
