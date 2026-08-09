use crate::{project::info::ProjectInfo, theme::Theme};
use gpui::*;

pub fn render(project_info: &ProjectInfo, license: &str, theme: &Theme) -> Div {
    super::section_box(
        "Metadata",
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(super::key_value(
                "Lines of Code:",
                project_info.line_count.to_string(),
                theme,
            ))
            .child(super::key_value(
                "Files:",
                project_info.file_count.to_string(),
                theme,
            ))
            .child(super::key_value(
                "Size:",
                project_info.project_size.clone(),
                theme,
            ))
            .child(super::key_value("License:", license.to_string(), theme)),
        theme,
    )
}
