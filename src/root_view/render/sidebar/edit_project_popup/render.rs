use crate::{
    config::Config,
    root_view::render::{
        self, input, sidebar::edit_project_popup::EditProjectPopup, steal_focus,
    },
};
use gpui::*;
use gpui_component::input::Input;

impl Render for EditProjectPopup {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.theme.get_theme();

        let project_name_input = Input::new(&self.project_name_input_state);

        let project_repo_input = Input::new(&self.project_repo_input_state);

        let cancle_button =
            render::text_button("cancle_button", "Cancle", None, &theme, None)
                .bg(theme.background_weak)
                .on_click(Self::close_button_pressed);

        let confirm_button =
            render::text_button("confirm_button", "Confirm", None, &theme, None)
                .bg(theme.special)
                .on_click(cx.listener(Self::confirm_button_pressed));

        let div = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.surface)
            .p_4()
            .gap_y_4()
            .text_color(theme.text)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_y_1()
                    .child("Project Name:")
                    .child(input!(project_name_input)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_y_1()
                    .child("Project Repo:")
                    .child(input!(project_repo_input)),
            )
            .child(div().flex_1())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(cancle_button)
                    .child(div().flex_1())
                    .child(confirm_button),
            );

        steal_focus!(cx, div)
    }
}
