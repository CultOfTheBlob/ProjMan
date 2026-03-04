use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, mouse_area, row, scrollable, space, text},
};

use crate::{message::Message, state::app_state::AppState};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    let mut project_list_content = column![].width(Length::Fill).spacing(4);

    for (index, project) in state.project_list.iter().enumerate()
    {
        let is_selected = state.selected_project == Some(index);

        let mut project_content = row![
            column![
                text(&project.name).style(text::primary),
                row![
                    text(format!("{:?}", &project.project_type)).style(text::secondary),
                    text(format!("{:?}", &project.path)).style(text::secondary)
                ]
                .spacing(32)
            ]
            .spacing(16),
            space().width(Length::Fill).height(48)
        ]
        .spacing(24);

        if !project.exists
        {
            project_content = row![
                column![
                    text("Missing!").style(text::danger).size(18),
                    text(format!("({})", &project.name))
                        .size(14)
                        .style(text::secondary),
                ]
                .spacing(8),
                space().width(Length::Fill).height(48)
            ]
            .spacing(24);

            if is_selected
            {
                project_content = project_content.push(
                    button(
                        if state.project_restoration_failed
                        {
                            text("Failed!")
                        }
                        else
                        {
                            text("Restore")
                        },
                    )
                    .style(
                        if state.project_restoration_failed
                        {
                            button::danger
                        }
                        else
                        {
                            button::primary
                        },
                    )
                    .on_press_maybe(
                        if state.pending.is_none() && !state.restoring_project
                        {
                            Some(Message::RestoreNonexistant)
                        }
                        else
                        {
                            None
                        },
                    ),
                );
                project_content = project_content.push(
                    button(text("Remove"))
                        .style(button::secondary)
                        .on_press_maybe(
                            if state.pending.is_none() && !state.restoring_project
                            {
                                Some(Message::RemoveNonexistant)
                            }
                            else
                            {
                                None
                            },
                        ),
                );
            }
        }

        let row_container = container(project_content)
            .width(Length::Fill)
            .padding(10)
            .style(
                if is_selected
                {
                    |theme: &Theme| container::Style {
                        background: Some(Color(theme.extended_palette().background.weaker.color)),
                        border: Border {
                            color: theme.extended_palette().background.strongest.color,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }
                else
                {
                    container::transparent
                },
            );

        let select_area = mouse_area(row_container)
            .on_press(Message::Select(index))
            .on_double_click(Message::Open(project.clone()));

        project_list_content = project_list_content.push(select_area);
    }

    let deselect_area =
        mouse_area(space().width(Length::Fill).height(Length::Fill)).on_press(Message::Deselect);

    let project_list = container(
        container(column![scrollable(project_list_content), deselect_area])
            .height(Length::Fill)
            .padding(16)
            .style(|theme: &Theme| container::Style {
                background: Some(Color(theme.extended_palette().background.base.color)),
                border: Border {
                    color: theme.extended_palette().background.strongest.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }),
    )
    .padding(12)
    .style(|theme: &Theme| container::Style {
        background: Some(Color(theme.extended_palette().background.weakest.color)),
        ..Default::default()
    });

    column![project_list, content].into()
}
