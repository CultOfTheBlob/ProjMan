use iced::{
    Background::Color,
    Border, Element, Length, Renderer, Theme,
    widget::{
        Container, button, checkbox, column, container, mouse_area, row, scrollable, stack, text,
    },
};

use crate::{app_state::AppState, message::Message};

pub fn view(state: &AppState) -> Element<'_, Message>
{
    let project_list_container = build_project_list_container(state);
    let top_bar_container = build_top_bar_container(state);

    let main_content = column![top_bar_container, project_list_container]
        .spacing(4)
        .into();

    handle_remove_popup(state, main_content)
}

fn handle_remove_popup<'a>(state: &AppState, content: Element<'a, Message>)
-> Element<'a, Message>
{
    if state.pending.is_none()
    {
        return content;
    }

    let popup_content: Container<'_, Message, Theme, Renderer> = container(
        column![
            text("Are you sure you want to remove this project?"),
            checkbox(state.delete_project_folder)
                .label("Also remove project folder")
                .on_toggle(Message::RemoveProjectFolder),
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
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    ]
    .into()
}

fn build_top_bar_container(state: &AppState) -> Container<'_, Message>
{
    let mut top_bar_content = row![
        button("Create")
            .style(button::secondary)
            .on_press(Message::Create),
        button("Import")
            .style(button::secondary)
            .on_press(Message::Import)
    ]
    .spacing(12);

    if let Some(index) = state.selected_project
    {
        top_bar_content = top_bar_content.push(
            button("Remove")
                .style(button::primary)
                .on_press(Message::Remove(index)),
        );
    }

    container(top_bar_content)
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
        })
}

fn build_project_list_container(state: &AppState) -> Container<'_, Message>
{
    let mut project_list_content = column![].spacing(4);

    for (index, project) in state.project_list.iter().enumerate()
    {
        let is_selected = state.selected_project == Some(index);

        let project_content = row![
            text(&project.name),
            text(format!("{:?}", &project.project_type))
        ]
        .spacing(24);

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

        let clickable = mouse_area(row_container)
            .on_press(Message::Selected(index))
            .on_double_click(Message::Open(project.clone()));

        project_list_content = project_list_content.push(clickable);
    }

    let project_list_content = scrollable(project_list_content);

    container(
        container(project_list_content)
            .height(Length::Fill)
            .padding(16)
            .style(|theme: &Theme| container::Style {
                background: Some(Color(theme.extended_palette().background.weakest.color)),
                border: Border {
                    color: theme.extended_palette().background.strongest.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },

                ..Default::default()
            }),
    )
    .padding(12)
}
