use crate::{
    config::Config,
    project::Project,
    root_view::render::{
        self, input, steal_focus, top_bar::import_project_popup::ImportProjectPopup,
    },
};
use gpui::{prelude::FluentBuilder, *};
use gpui_component::input::Input;

impl Render for ImportProjectPopup {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.theme.get_theme();

        let projects_directory = &cx.global::<Config>().general.projects_dir;

        let project_path_input_value = self.project_path_input_state.read(cx).value();

        let project_path_is_valid = {
            let is_project_file =
                project_path_input_value.ends_with(Project::<()>::PROJECT_FILE_NAME);

            let is_under_projects_directory =
                project_path_input_value.starts_with(projects_directory);

            is_project_file && is_under_projects_directory
        };

        let project_path_input = Input::new(&self.project_path_input_state);

        let project_path_invalid_text = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_x_1()
            .text_color(theme.error)
            .child(div().text_size(px(24.0)).child(""))
            .child("This path is not a valid project!");

        let select_directory_button =
            render::text_button("select_directory_button", "󰝰", None, &theme, None)
                .bg(theme.background_weak)
                .text_color(theme.text_muted)
                .on_click(cx.listener(Self::select_directory_button_pressed));

        let cancle_button =
            render::text_button("cancle_button", "Cancle", None, &theme, None)
                .bg(theme.background_weak)
                .on_click(Self::close_button_pressed);

        let confirm_button = render::text_button(
            "confirm_button",
            "Confirm",
            None,
            &theme,
            Some(!project_path_is_valid),
        )
        .bg(theme.special)
        .when_else(
            !project_path_is_valid,
            |this: Stateful<Div>| {
                this.bg(theme.background)
                    .text_color(theme.text_disabled)
                    .border_color(theme.error)
            },
            |this: Stateful<Div>| {
                this.on_click(cx.listener(Self::confirm_button_pressed))
            },
        );

        let div = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.surface)
            .p_4()
            .text_color(theme.text)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_y_4()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_x_0p5()
                            .child(div().flex_1().child(input!(project_path_input)))
                            .child(select_directory_button),
                    )
                    .when(!project_path_is_valid, |this: Div| {
                        this.child(project_path_invalid_text)
                    }),
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
