use std::path::PathBuf;

use iced::{
    Background::Color,
    Border, Element, Length, Shadow, Theme, Vector,
    widget::{
        Text, button, column, combo_box, container, progress_bar, row,
        scrollable::{self, Scrollable},
        stack, text, text_input,
    },
};

use crate::{
    message::Message,
    project::project_creator,
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
                if !state.project_creation_status.creating
                {
                    Some(Message::NewProjectNameChanged)
                }
                else
                {
                    None
                }
            )
        ]
        .spacing(4),
    );

    let project_type_widget = if !state.project_creation_status.creating
    {
        container(
            column![
                text("Project Type:"),
                combo_box(
                    &state.project_types,
                    "",
                    Some(&state.new_project.project_type),
                    Message::NewProjectTypeChanged
                )
            ]
            .spacing(4),
        )
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
    };

    let project_repo_widget = container(
        column![
            text("Project Repo:"),
            text_input(
                "https://github.com/Author/Project.git",
                &state.new_project.repo
            )
            .on_input_maybe(
                if !state.project_creation_status.creating
                {
                    Some(Message::NewProjectRepoChanged)
                }
                else
                {
                    None
                }
            ),
        ]
        .spacing(4),
    );

    let project_path_widget = container(
        column![
            text("Project Path:"),
            column![
                text_input("", &state.new_project.path.to_string_lossy())
                    .on_input_maybe(
                        if !state.project_creation_status.creating
                        {
                            Some(|path| Message::NewProjectPathChanged(PathBuf::from(path)))
                        }
                        else
                        {
                            None
                        }
                    )
                    .style(|theme: &Theme, status| {
                        let mut style = text_input::default(theme, status);

                        if !state.project_creation_status.creating
                            && state
                                .new_project
                                .path_is_valid(&state.config.general.projects_dir)
                                .is_err()
                        {
                            style.border.color = theme.extended_palette().danger.base.color
                        }

                        style
                    }),
                text(
                    if !state.project_creation_status.creating
                        && let Err(err) = state
                            .new_project
                            .path_is_valid(&state.config.general.projects_dir)
                    {
                        String::from(" ") + &err
                    }
                    else
                    {
                        String::new()
                    }
                )
                .height(
                    if !state.project_creation_status.creating
                        && state
                            .new_project
                            .path_is_valid(&state.config.general.projects_dir)
                            .is_ok()
                    {
                        Length::from(0)
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
    );

    let console_widget = Scrollable::new(column(
        state
            .project_creation_status
            .log
            .iter()
            .enumerate()
            .map(|(i, l)| {
                Text::new(l)
                    .style(
                        if state.project_creation_status.failed
                            && i == state.project_creation_status.log.len() - 1
                        {
                            text::danger
                        }
                        else
                        {
                            text::success
                        },
                    )
                    .into()
            }),
    ))
    .width(Length::Fill)
    .height(96)
    .style(
        |theme: &Theme, status: scrollable::Status| scrollable::Style {
            container: container::Style {
                background: Some(Color(theme.extended_palette().background.weaker.color)),
                border: Border {
                    color: theme.extended_palette().background.strong.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            },
            ..scrollable::default(theme, status)
        },
    );

    let copy_widget = container(
        button("")
            .on_press_maybe(
                if state.project_creation_status.failed && !state.project_creation_status.creating
                {
                    Some(Message::CreationErrorCopied)
                }
                else
                {
                    None
                },
            )
            .style(|theme: &Theme, status: button::Status| match status
            {
                button::Status::Disabled => button::Style {
                    background: Some(Color(iced::Color::TRANSPARENT)),
                    text_color: iced::Color::TRANSPARENT,
                    ..button::secondary(theme, status)
                },

                _ => button::Style {
                    text_color: theme.extended_palette().background.weak.color,
                    border: Border {
                        color: theme.extended_palette().background.strong.color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    shadow: Shadow {
                        color: theme.extended_palette().background.weakest.color,
                        offset: Vector { x: 3.0, y: 3.0 },
                        blur_radius: 6.0,
                    },
                    ..button::secondary(theme, status)
                },
            }),
    )
    .padding(8)
    .align_right(Length::Fill)
    .align_top(Length::Fill);

    let progress_widget = container(
        column![
            progress_bar(
                0.0..=project_creator::STEPS,
                state.project_creation_status.step
            ),
            stack![console_widget, copy_widget]
        ]
        .spacing(8),
    );

    let cancel_widget = button("Cancel").style(button::secondary).on_press_maybe(
        if !state.project_creation_status.creating
        {
            Some(Message::CreateCanceled)
        }
        else
        {
            None
        },
    );

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
            if !state.project_creation_status.creating
                && state
                    .new_project
                    .path_is_valid(&state.config.general.projects_dir)
                    .is_ok()
            {
                Some(Message::CreateConfirmed)
            }
            else
            {
                None
            },
        );

    let popup_content = container(column![
        title_bar,
        project_name_widget.padding(16),
        project_type_widget.padding(16),
        project_repo_widget.padding(16),
        project_path_widget.padding(16),
        progress_widget.padding(16),
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
                background: Some(Color(iced::Color {
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
