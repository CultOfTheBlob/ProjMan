use crate::{
    app_state::GlobalAppState,
    config::Config,
    project::valid_project::ValidProject,
    root_view::render::{self, sidebar::remove_project_popup::RemoveProjectPopup},
};
use gpui::{prelude::FluentBuilder, *};
use gpui_component::checkbox::Checkbox;

impl Render for RemoveProjectPopup {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.theme.get_theme();
        let app_state = cx.global::<GlobalAppState>().0.clone();

        let project = app_state.get_selected_project();

        let project_exists = matches!(project, Some(ValidProject::Existant(_)));

        let remove_message = "Are you sure you want to remove this project?";

        let cancle_button =
            render::text_button("cancle_button", "Cancle", None, &theme, None)
                .bg(theme.background_weak)
                .on_click(Self::close_button_pressed);

        let confirm_button =
            render::text_button("confirm_button", "Confirm", None, &theme, None)
                .bg(theme.error)
                .on_click(cx.listener(Self::confirm_button_pressed));

        let remove_folder_checkbox = {
            let checkbox = Checkbox::new("remove_folder_checkbox_toggle")
                .checked(self.remove_folder_checked)
                .border_1()
                .rounded_md()
                .group_hover("remove_folder_checkbox", |style: StyleRefinement| {
                    style.border_color(theme.special)
                });

            let label = "Also remove project folder";

            div()
                .id("remove_folder_checkbox")
                .group("remove_folder_checkbox")
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(checkbox)
                .child(label)
                .text_color(theme.text_muted)
                .on_click(cx.listener(Self::remove_folder_checkbox_checked))
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.surface)
            .p_4()
            .gap_y_3()
            .text_color(theme.text)
            .child(remove_message)
            .when(project_exists, |this: Div| {
                this.child(remove_folder_checkbox)
            })
            .child(div().flex_1())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(cancle_button)
                    .child(div().flex_1())
                    .child(confirm_button),
            )
    }
}
