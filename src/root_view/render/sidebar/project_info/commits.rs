use crate::theme::Theme;
use gpui::*;

pub fn render(last_commit: &str, commit_count: usize, theme: &Theme) -> Div {
    super::section_box(
        "Commits",
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(super::key_value(
                "Last Commit:",
                last_commit.to_string(),
                theme,
            ))
            .child(super::key_value(
                "Number of Commits:",
                commit_count.to_string(),
                theme,
            )),
        theme,
    )
}
