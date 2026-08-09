use crate::theme::Theme;
use gpui::*;

pub fn render(authors: &[(String, f32)], theme: &Theme) -> Div {
    super::section_box(
        "Authors",
        div()
            .flex()
            .flex_col()
            .gap_2()
            .children(authors.iter().map(|(author, pct)| {
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_color(theme.accent).child(""))
                    .child(div().text_color(theme.text).child(author.clone()))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(format!("({pct:.1}%)")),
                    )
            })),
        theme,
    )
}
