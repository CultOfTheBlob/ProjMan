use crate::{
    app_state::GlobalAppState,
    project::valid_project::ValidProject,
    root_view::{
        RootView, popup::Popup, render::sidebar::edit_project_popup::EditProjectPopup,
    },
};
use gpui::*;
use gpui_component::input::InputState;

impl Popup for EditProjectPopup {
    const TITLE: &'static str = "Edit Project";

    const ID: &'static str = "projman.popup.editproject";

    const WIDTH_FRACTION: f32 = 0.20;

    const HEIGHT_FRACTION: f32 = 0.20;

    fn create(_root_view: &Entity<RootView>, window: &mut Window, cx: &mut App) -> Self {
        let app_state = cx.global::<GlobalAppState>().0.clone();

        let selected_project = app_state.get_selected_project();

        let (name, repo) = match &selected_project {
            Some(ValidProject::Existant(project)) => {
                (project.name.as_str(), project.repo.as_str())
            }
            _ => ("", ""),
        };

        let project_name_input_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx)
                .placeholder("...")
                .default_value(name.to_string())
        });

        let project_repo_input_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx)
                .placeholder("...")
                .default_value(repo.to_string())
        });

        Self {
            focus_handle: cx.focus_handle(),

            project_name_input_state,
            project_repo_input_state,
        }
    }
}
