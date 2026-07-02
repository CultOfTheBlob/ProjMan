use crate::{
    message::Message,
    state::app_state::{AppState, NotifKind},
};
use iced::{
    Alignment,
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row, stack, text},
};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message> {
    if state.notifications.is_empty() {
        return content;
    }

    let mut notif_widget = column![].spacing(16).padding(12);

    for (index, notification) in state.notifications.iter().enumerate() {
        let mut new_notif = text(&notification.text);

        match &notification.kind {
            NotifKind::Warning => new_notif = new_notif.style(text::warning),
            NotifKind::Error => new_notif = new_notif.style(text::danger),
        }

        let notif_icon = match &notification.kind {
            NotifKind::Warning => text("").size(32).style(text::warning),
            NotifKind::Error => text("").size(32).style(text::danger),
        };

        notif_widget = notif_widget.push(
            container(
                row![
                    notif_icon,
                    new_notif.wrapping(text::Wrapping::Glyph),
                    row![
                        button("")
                            .on_press(Message::NotificationCopied(index))
                            .style(|theme: &Theme, status: button::Status| {
                                button::Style {
                                    text_color: theme.extended_palette().background.strongest.color,
                                    border: Border {
                                        color: theme.extended_palette().background.base.color,
                                        width: 0.0,
                                        radius: 4.0.into(),
                                    },

                                    ..button::subtle(theme, status)
                                }
                            }),
                        button("")
                            .on_press(Message::NotificationRemoved(index))
                            .style(|theme: &Theme, status: button::Status| {
                                button::Style {
                                    text_color: theme.extended_palette().background.strongest.color,
                                    border: Border {
                                        color: theme.extended_palette().background.base.color,
                                        width: 0.0,
                                        radius: 4.0.into(),
                                    },

                                    ..button::subtle(theme, status)
                                }
                            })
                    ]
                    .padding(4)
                    .spacing(2),
                ]
                .align_y(Alignment::Center)
                .spacing(8)
                .wrap(),
            )
            .padding(12)
            .max_width(1024)
            .style(|theme: &Theme| container::Style {
                background: Some(Color({
                    let color = theme.extended_palette().background.weaker.color;

                    iced::Color {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                        a: 0.5,
                    }
                })),
                border: Border {
                    color: theme.extended_palette().background.strongest.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }),
        );
    }

    let popup_content = container(notif_widget)
        .width(Length::Shrink)
        .style(|theme: &Theme| container::Style {
            background: Some(Color({
                let color = theme.extended_palette().background.weakest.color;

                iced::Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: 0.5,
                }
            })),
            border: Border {
                color: theme.extended_palette().background.strongest.color,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        });

    stack![
        content,
        container(popup_content)
            .padding(24)
            .align_right(Length::Fill)
            .align_bottom(Length::Fill),
    ]
    .into()
}
