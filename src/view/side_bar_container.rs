use iced::{
    Alignment,
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row, text},
};

use crate::{message::Message, state::app_state::AppState};

pub fn build<'a>(state: &AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    let mut top_bar_content = column![].spacing(12);

    top_bar_content = top_bar_content.push(
        button(
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
        ),
    );

    top_bar_content = top_bar_content.push(
        button(
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
        ),
    );

    top_bar_content = top_bar_content.push(
        button(
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
        ),
    );

    top_bar_content = top_bar_content.push(
        button(
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
        ),
    );

    let top_bar = container(top_bar_content)
        .width(Length::FillPortion(1))
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
