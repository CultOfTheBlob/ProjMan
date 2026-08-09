use crate::{
    app_state::{AppState, GlobalAppState},
    config::Config,
    log::Log,
    project::{Nonexistant, Project},
    root_view::{RootView, render},
};
use gpui::{prelude::FluentBuilder, *};
use gpui_component::{Icon, IconName, spinner::Spinner};
use std::sync::Arc;

pub fn render(
    cx: &Context<RootView>,
    project: &Arc<Project<Nonexistant>>,
    is_selected: bool,
    index: usize,
) -> Stateful<Div> {
    let root_view = cx.entity();
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    let bg_color = if is_selected {
        theme.surface_strong
    } else {
        theme.background
    };

    let border_color = if is_selected {
        theme.accent
    } else {
        theme.background_weak
    };

    let set_selected_project_index = |index: Option<usize>| {
        move |_: &mut RootView, cx: &mut Context<RootView>| {
            AppState::set_selected_project_index(cx, index);

            cx.notify();
        }
    };

    let listener = {
        move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            root_view.update(cx, set_selected_project_index(Some(index)));

            cx.stop_propagation();
        }
    };

    let icon = div()
        .flex()
        .flex_row()
        .items_center()
        .size_16()
        .text_color(theme.error)
        .child(Icon::new(IconName::CircleX).size_16());

    let title = div().text_color(theme.error).child("Missing!");

    let project_name = div()
        .text_color(theme.text_muted)
        .child(project.name.clone());

    let restore_button = {
        let listener = {
            let project = project.clone();

            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                AppState::set_restoring_project(cx, true);

                let project = project.clone();

                let restore = |cx: AsyncApp| async move {
                    let restored_project = cx
                        .background_executor()
                        .spawn(async move { Arc::unwrap_or_clone(project).restore() })
                        .await;

                    let _ = cx.update_global::<GlobalAppState, ()>(|app_state, _| {
                        let app_state = Arc::make_mut(&mut app_state.0);

                        match restored_project {
                            Ok(restored) => {
                                if let Err(err) = app_state.restore_project(restored) {
                                    Log::Error.log(&err.to_string());
                                }
                            }
                            Err(err) => {
                                Log::Error.log(&err.to_string());
                            }
                        }

                        app_state.restoring_project = false;
                    });
                };

                cx.spawn(|cx: &mut AsyncApp| {
                    let cx = cx.clone();

                    restore(cx)
                })
                .detach();
            }
        };

        let button_label = if app_state.restoring_project {
            "Restoring..."
        } else {
            "Restore"
        };

        let spinner = Spinner::new()
            .color(theme.accent_alt.into())
            .icon(IconName::LoaderCircle);

        render::text_button(
            "restore_button",
            button_label,
            None,
            &theme,
            Some(app_state.restoring_project),
        )
        .border_color(theme.accent_alt)
        .when(app_state.restoring_project, |this: Stateful<Div>| {
            this.gap_x_2().child(spinner)
        })
        .when(!app_state.restoring_project, |this: Stateful<Div>| {
            this.on_click(listener)
        })
    };

    div()
        .id(SharedString::from(format!("project_{}", project.name)))
        .flex()
        .flex_row()
        .w_full()
        .items_center()
        .p_4()
        .gap_4()
        .bg(bg_color)
        .border_1()
        .border_color(border_color)
        .rounded_lg()
        .cursor_pointer()
        .hover(|style: StyleRefinement| style.bg(theme.surface))
        .on_click(listener)
        .child(icon)
        .child(
            div()
                .flex()
                .flex_row()
                .w_full()
                .gap_4()
                .child(title)
                .child(div().text_color(theme.text_muted).child(project_name)),
        )
        .child(div().flex_1())
        .when(is_selected, |this: Stateful<Div>| {
            this.child(restore_button)
        })
}
