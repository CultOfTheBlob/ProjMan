use crate::{
    app_state::GlobalAppState,
    config::Config,
    root_view::{
        RootView, popup::Popup, render::top_bar::create_project_popup::CreateProjectPopup,
    },
};
use gpui::*;
use gpui_component::{
    IndexPath,
    input::InputState,
    select::{SearchableVec, SelectState},
};

impl Popup for CreateProjectPopup {
    const TITLE: &'static str = "Create Project";

    const ID: &'static str = "projman.popup.createproject";

    const WIDTH_FRACTION: f32 = 0.30;

    const HEIGHT_FRACTION: f32 = 0.40;

    fn create(_root_view: &Entity<RootView>, window: &mut Window, cx: &mut App) -> Self {
        let app_state = cx.global::<GlobalAppState>().0.clone();

        let project_name_input_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx)
                .placeholder("...")
                .default_value("NewProject")
        });

        let project_repo_input_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx)
                .placeholder("git@github.com:Author/NewProject.git")
        });

        let project_template_select_state =
            cx.new(|cx: &mut Context<SelectState<SearchableVec<String>>>| {
                let template_names = app_state
                    .templates
                    .keys()
                    .map(String::clone)
                    .collect::<Vec<String>>();

                SelectState::new(
                    SearchableVec::new(template_names),
                    Some(IndexPath::default()),
                    window,
                    cx,
                )
                .searchable(true)
            });

        let project_path_input_state = cx.new(|cx: &mut Context<InputState>| {
            let config = cx.global::<Config>();

            let mut default_path = config.general.projects_dir.clone();
            default_path.push_str("NewProject");

            InputState::new(window, cx)
                .placeholder("...")
                .default_value(default_path)
        });

        Self {
            focus_handle: cx.focus_handle(),

            scroll_handle: ScrollHandle::new(),

            project_name_input_state,
            project_repo_input_state,
            project_template_select_state,
            project_path_input_state,

            console_logs: vec![],
        }
    }
}
