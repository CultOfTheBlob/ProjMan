use crate::theme::Theme;
use gpui::*;

pub fn render(branches: &[String], current_branch: usize, theme: &Theme) -> Div {
    super::section_box(
        "Branches",
        div()
            .flex()
            .flex_col()
            .gap_1_5()
            .children(branches.iter().enumerate().map(|(index, branch)| {
                let is_current = current_branch == index;

                let connector = match index {
                    0 if branches.len() == 1 => "──",
                    0 => "╭─",
                    i if i == branches.len() - 1 => "╰─",
                    _ => "├─",
                };

                let dot = if is_current { "" } else { "" };
                let color = if is_current {
                    theme.accent
                } else {
                    theme.text_muted
                };

                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_color(color)
                    .child(connector)
                    .child(dot)
                    .child(branch.clone())
            })),
        theme,
    )
}
