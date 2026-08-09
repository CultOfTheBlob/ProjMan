use crate::theme::Theme;
use gpui::*;
use tokei::LanguageType;

pub fn render(language_percentages: &[(LanguageType, f32)], theme: &Theme) -> Div {
    super::section_box(
        "Languages",
        div()
            .flex()
            .flex_col()
            .gap_2_5()
            .children(language_percentages.iter().map(|(lang, pct)| {
                let bar_width = pct.clamp(0.0, 100.0) / 100.0;

                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_color(theme.accent).child(""))
                            .child(
                                div()
                                    .text_color(theme.accent)
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(lang.to_string()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(format!("({pct:.1}%)")),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .max_w(px(140.0))
                            .h(px(6.0))
                            .bg(theme.surface)
                            .rounded_full()
                            .overflow_hidden()
                            .child(
                                div()
                                    .h_full()
                                    .w(relative(bar_width))
                                    .bg(theme.accent)
                                    .rounded_full(),
                            ),
                    )
            })),
        theme,
    )
}
