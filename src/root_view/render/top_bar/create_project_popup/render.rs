use std::{fs, path::PathBuf};

use crate::{
    app_state::GlobalAppState,
    config::Config,
    root_view::render::{
        self, input, steal_focus, top_bar::create_project_popup::CreateProjectPopup,
    },
    utils,
};
use gpui::{prelude::FluentBuilder as _, *};
use gpui_component::{input::Input, scroll::ScrollableElement, select::Select};

impl Render for CreateProjectPopup {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.theme.get_theme();
        let app_state = cx.global::<GlobalAppState>().0.clone();

        let creating_project = app_state.creating_project;

        let project_path_input_value = self.project_path_input_state.read(cx).value();

        let project_path_is_valid = {
            let projects_directory = &cx.global::<Config>().general.projects_dir;

            let exists = PathBuf::from(project_path_input_value.to_string()).exists();

            let is_empty = fs::read_dir(project_path_input_value.to_string())
                .is_ok_and(|mut entries| entries.next().is_none());

            let is_under_projects_directory =
                project_path_input_value.starts_with(projects_directory);

            !exists || (is_empty && is_under_projects_directory)
        };

        let project_name_input =
            Input::new(&self.project_name_input_state).disabled(creating_project);

        let project_repo_input =
            Input::new(&self.project_repo_input_state).disabled(creating_project);

        let project_template_select = Select::new(&self.project_template_select_state)
            .placeholder("...")
            .disabled(creating_project);

        let project_path_input =
            Input::new(&self.project_path_input_state).disabled(creating_project);

        let project_path_invalid_text = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_x_1()
            .text_color(theme.error)
            .child(div().text_size(px(24.0)).child(""))
            .child("This path is not a valid project!");

        let select_directory_button = render::text_button(
            "select_directory_button",
            "󰝰",
            None,
            &theme,
            Some(creating_project),
        )
        .bg(theme.background_weak)
        .text_color(theme.text_muted)
        .when_else(
            !project_path_is_valid,
            |this: Stateful<Div>| {
                this.bg(theme.background).text_color(theme.text_disabled)
            },
            |this: Stateful<Div>| {
                this.on_click(cx.listener(Self::select_directory_button_pressed))
            },
        );

        let console = div()
            .id("console")
            .size_full()
            .h(px(480.0))
            .mt_5()
            .bg(theme.background)
            .text_color(theme.text_muted)
            .border_1()
            .rounded_sm()
            .border_color(theme.border)
            .scrollbar(&self.scroll_handle, Axis::Vertical)
            .track_scroll(&self.scroll_handle)
            .overflow_y_scroll()
            .children(self.console_logs.iter().map(|line| {
                let logs = utils::parse_ansi(line, &theme);

                div()
                    .flex()
                    .flex_row()
                    .whitespace_nowrap()
                    .children(logs.into_iter().map(Div::from))
            }));

        let cancle_button = render::text_button(
            "cancle_button",
            "Cancle",
            None,
            &theme,
            Some(creating_project),
        )
        .bg(theme.background_weak)
        .when_else(
            !project_path_is_valid,
            |this: Stateful<Div>| {
                this.bg(theme.background).text_color(theme.text_disabled)
            },
            |this: Stateful<Div>| this.on_click(Self::close_button_pressed),
        );

        let confirm_button = render::text_button(
            "confirm_button",
            "Confirm",
            None,
            &theme,
            Some(!project_path_is_valid && creating_project),
        )
        .bg(theme.special)
        .when_else(
            !project_path_is_valid,
            |this: Stateful<Div>| {
                this.bg(theme.background)
                    .text_color(theme.text_disabled)
                    .when(!creating_project, |this: Stateful<Div>| {
                        this.border_color(theme.error)
                    })
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
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_y_1()
                    .child("Project Template:")
                    .child(project_template_select),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_y_1()
                    .child("Project Path:")
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_x_0p5()
                            .child(div().flex_1().child(input!(project_path_input)))
                            .child(select_directory_button),
                    )
                    .when(!project_path_is_valid && !creating_project, |this: Div| {
                        this.child(project_path_invalid_text)
                    }),
            )
            .child(console)
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
