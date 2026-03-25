use iced::{
    Alignment,
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row, space, text},
};

use crate::{message::Message, project::Project, state::app_state::AppState};

pub fn build<'a>(state: &AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    if !state.sidebar_expanded
    {
        return row![content].padding(4).into();
    }

    let open_widget = button(
        row![text("").size(24), text("Open")]
            .spacing(4)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .style(|theme: &Theme, status: button::Status| match status
    {
        button::Status::Disabled => button::subtle(theme, status),

        _ => button::secondary(theme, status),
    })
    .on_press_maybe(
        if state.pending.is_none()
            && let Some(index) = state.selected_project
            && state.project_list[index].exists
        {
            Some(Message::Opened(state.project_list[index].clone()))
        }
        else
        {
            None
        },
    );

    let edit_widget = button(
        row![text("").size(24), text("Edit")]
            .spacing(4)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .style(|theme: &Theme, status: button::Status| match status
    {
        button::Status::Disabled => button::subtle(theme, status),

        _ => button::secondary(theme, status),
    })
    .on_press_maybe(
        if state.pending.is_none()
            && let Some(index) = state.selected_project
            && state.project_list[index].exists
        {
            Some(Message::Edited(index))
        }
        else
        {
            None
        },
    );

    let update_widget = button(
        row![text("").size(24), text("Update")]
            .spacing(4)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .style(|theme: &Theme, status: button::Status| match status
    {
        button::Status::Disabled => button::subtle(theme, status),

        _ => button::primary(theme, status),
    })
    .on_press_maybe(
        if state.pending.is_none()
            && let Some(index) = state.selected_project
            && state.project_list[index].exists
            && state.project_list[index].is_outdated()
        {
            Some(Message::Updated)
        }
        else
        {
            None
        },
    );

    let remove_widget = button(
        row![text("󰆴").size(24), text("Remove")]
            .spacing(4)
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .style(|theme: &Theme, status: button::Status| match status
    {
        button::Status::Disabled => button::subtle(theme, status),

        _ => button::warning(theme, status),
    })
    .on_press_maybe(
        if state.pending.is_none() && state.selected_project.is_some()
        {
            Some(Message::Removed)
        }
        else
        {
            None
        },
    );

    let info_widget = {
        let title_bar_widget = container(text("Project Information:"))
            .width(Length::Fill)
            .padding(8)
            .style(|theme: &Theme| container::Style {
                background: Some(Color(theme.extended_palette().background.strong.color)),
                border: Border {
                    color: theme.extended_palette().background.strongest.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            });

        let mut info_column_widget = column![title_bar_widget];

        if let Some(index) = state.selected_project
        {
            let project: &Project = &state.project_list[index];

            if let Some(project_info) = project.info()
            {
                let repo_widget = row![
                    text("Repo:").style(|theme: &Theme| text::Style {
                        color: Some(theme.extended_palette().secondary.strong.color)
                    }),
                    button(text(project.repo.to_string()))
                        .style(button::text)
                        .on_press_maybe(
                            if state.pending.is_none()
                            {
                                Some(Message::RepoOpened)
                            }
                            else
                            {
                                None
                            }
                        )
                ]
                .padding(8)
                .align_y(Alignment::Center);

                let branch_widget = {
                    let mut branch_column_widget = column![].spacing(0).padding(8);

                    for (index, branch) in project_info.branches.iter().enumerate()
                    {
                        let is_current: bool = project_info.current_branch == index;

                        let style_text = |s: String| {
                            let text = text(s);
                            if is_current
                            {
                                text.style(text::primary)
                            }
                            else
                            {
                                text
                            }
                        };

                        let connector: &str = match index
                        {
                            0 => "╭─",
                            i if i == project_info.branches.len() - 1 => "╰─",
                            _ => "├─",
                        };

                        let dot: &str = { if is_current { "" } else { "" } };

                        branch_column_widget = branch_column_widget.push(
                            row![
                                style_text(connector.to_string()),
                                style_text(dot.to_string()),
                                space().width(12),
                                style_text(branch.to_string())
                            ]
                            .align_y(Alignment::Center),
                        );
                    }

                    branch_column_widget
                };

                let languages_widget = {
                    let mut language_column_widget = column![].spacing(8).padding(8);

                    let max_label_len: usize = project_info
                        .language_percentage
                        .iter()
                        .map(|(l, p)| l.name().len() + format!("({:.1}%)", p).len())
                        .max()
                        .unwrap_or(0);

                    for (language_type, percentage) in &project_info.language_percentage
                    {
                        language_column_widget = language_column_widget.push(
                            row![
                                row![
                                    text("").size(18).style(text::primary),
                                    text(language_type.name()),
                                    text(format!("({:.1}%)", &percentage))
                                        .style(text::secondary)
                                        .size(14)
                                ]
                                .width(Length::Fixed(max_label_len as f32 * 10.0))
                                .spacing(4)
                                .align_y(Alignment::Center),
                                container("")
                                    .height(10)
                                    .width(*percentage as f32 * 2.0)
                                    .style(|theme: &Theme| {
                                        container::Style {
                                            background: Some(Color(
                                                theme.extended_palette().primary.base.color,
                                            )),
                                            border: Border {
                                                color: theme.extended_palette().primary.base.color,
                                                width: 1.0,
                                                radius: 10.into(),
                                            },

                                            ..container::transparent(theme)
                                        }
                                    })
                            ]
                            .spacing(16)
                            .align_y(Alignment::Center),
                        );
                    }

                    language_column_widget
                };

                let authors_widget = {
                    let mut authors_row_widget = row![].spacing(8).padding(8);

                    for (author, percentage) in project_info.authors
                    {
                        authors_row_widget = authors_row_widget.push(
                            row![
                                text("").size(18).style(text::primary),
                                text(author),
                                text(format!("({:.1}%)", &percentage))
                                    .style(text::secondary)
                                    .size(14)
                            ]
                            .spacing(4)
                            .align_y(Alignment::Center),
                        );
                    }

                    authors_row_widget.wrap()
                };

                let last_commit_widget = row![
                    text("Last Commit:").style(|theme: &Theme| text::Style {
                        color: Some(theme.extended_palette().secondary.strong.color)
                    }),
                    text(project_info.last_commit).wrapping(text::Wrapping::Glyph)
                ]
                .spacing(4)
                .padding(8);

                let commit_count_widget = row![
                    text("Number of Commits:").style(|theme: &Theme| text::Style {
                        color: Some(theme.extended_palette().secondary.strong.color)
                    }),
                    text(project_info.commit_count)
                ]
                .spacing(4)
                .padding(8);

                let line_count_widget = row![
                    text("Lines of Code:").style(|theme: &Theme| text::Style {
                        color: Some(theme.extended_palette().secondary.strong.color)
                    }),
                    text(project_info.line_count)
                ]
                .spacing(4)
                .padding(8);

                let file_count_widget = row![
                    text("Files:").style(|theme: &Theme| text::Style {
                        color: Some(theme.extended_palette().secondary.strong.color)
                    }),
                    text(project_info.file_count)
                ]
                .spacing(4)
                .padding(8);

                let project_size_widget = row![
                    text("Size:").style(|theme: &Theme| text::Style {
                        color: Some(theme.extended_palette().secondary.strong.color)
                    }),
                    text(format!("{}", project_info.project_size.display().iec()))
                ]
                .spacing(4)
                .padding(8);

                let devider_line = |title: &'a str| {
                    row![
                        container("")
                            .height(1)
                            .width(Length::Fill)
                            .style(|theme: &Theme| container::Style {
                                background: Some(Color(
                                    theme.extended_palette().secondary.strong.color,
                                )),
                                border: Border {
                                    color: theme.extended_palette().secondary.strong.color,
                                    width: 1.0,
                                    radius: 20.into(),
                                },

                                ..container::transparent(theme)
                            }),
                        text(title).style(|theme: &Theme| text::Style {
                            color: Some(theme.extended_palette().secondary.strong.color)
                        }),
                        container("")
                            .height(1)
                            .width(Length::Fill)
                            .style(|theme: &Theme| container::Style {
                                background: Some(Color(
                                    theme.extended_palette().secondary.strong.color,
                                )),
                                border: Border {
                                    color: theme.extended_palette().secondary.strong.color,
                                    width: 1.0,
                                    radius: 20.into(),
                                },

                                ..container::transparent(theme)
                            })
                    ]
                    .spacing(1)
                    .align_y(Alignment::Center)
                };

                info_column_widget = info_column_widget.push(
                    column![
                        repo_widget,
                        devider_line("<Branches>"),
                        branch_widget,
                        devider_line("<Languages>"),
                        languages_widget,
                        devider_line("<Authors>"),
                        authors_widget,
                        devider_line("<Commits>"),
                        last_commit_widget,
                        commit_count_widget,
                        devider_line("<Metadata>"),
                        line_count_widget,
                        file_count_widget,
                        project_size_widget
                    ]
                    .padding(4),
                )
            }
        }

        container(info_column_widget)
            .width(Length::Fill)
            .style(|theme: &Theme| container::Style {
                border: Border {
                    color: theme.extended_palette().background.strongest.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
    };

    let top_bar_content = column![
        open_widget,
        edit_widget,
        update_widget,
        remove_widget,
        space().height(32),
        info_widget
    ]
    .spacing(12);

    let top_bar = container(top_bar_content)
        .width(Length::FillPortion(3))
        .height(Length::Fill)
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

    row![content, top_bar].padding(4).into()
}
