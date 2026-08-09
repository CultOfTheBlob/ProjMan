use crate::{
    app_state::{AppState, GlobalAppState},
    config::Config,
    log::Log,
    project::valid_project::ValidProject,
    root_view::RootView,
};
use gpui::*;
use gpui_component::input::InputState;

mod existant_project;
mod nonexistant_project;

pub fn render(cx: &Context<RootView>, search_bar_state: &InputState) -> Div {
    let root_view = cx.entity();
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    let projects = &app_state.get_filtered_projects(&search_bar_state.value());

    let selected_project_index = app_state.selected_project_index;

    let project_list = div().flex().flex_col().gap_y_2p5().children(
        projects
            .iter()
            .enumerate()
            .filter_map(|(index, project)| match project {
                ValidProject::Existant(project) => {
                    let template = project
                        .get_template(&app_state)
                        .map_err(|err| {
                            Log::Error.log(&err.to_string());
                        })
                        .ok()?;

                    let icon = &template.icon_path;

                    let is_selected = selected_project_index.is_some_and(|i| i == index);

                    Some(existant_project::render(
                        cx,
                        project,
                        icon,
                        is_selected,
                        index,
                    ))
                }
                ValidProject::Nonexistant(project) => {
                    let is_selected = selected_project_index.is_some_and(|i| i == index);

                    Some(nonexistant_project::render(cx, project, is_selected, index))
                }
            }),
    );

    let listener = move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
        root_view.update(cx, |_, cx: &mut Context<RootView>| {
            AppState::set_selected_project_index(cx, None);

            cx.notify();
        });
    };

    div().flex_1().h_full().p_2().child(
        div()
            .id("project_list")
            .size_full()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .bg(theme.background)
            .rounded_lg()
            .border_1()
            .border_color(theme.border)
            .p_4()
            .text_color(theme.text)
            .child(project_list)
            .on_click(listener),
    )
}
