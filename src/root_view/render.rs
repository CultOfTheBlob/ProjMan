use crate::{
    app_state::GlobalAppState, config::Config, root_view::RootView, theme::Theme,
};
use gpui::{prelude::FluentBuilder, *};
use gpui_animation::{
    animation::TransitionExt as _, transition::general::EaseInOutCubic,
};
use std::time::Duration;

mod project_list;
mod sidebar;
mod top_bar;

macro_rules! input {
    ($input:expr) => {
        div()
            .child($input)
            .on_mouse_down(MouseButton::Left, move |_, _, cx: &mut App| {
                cx.stop_propagation();
            })
    };
}

pub(crate) use input;

macro_rules! steal_focus {
    ($context:expr, $div:expr) => {
        $div.on_mouse_down(
            MouseButton::Left,
            $context.listener(move |this: &mut Self, _, window: &mut Window, _| {
                this.focus_handle.focus(window);
            }),
        )
    };
}

pub(crate) use steal_focus;

impl Render for RootView {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.theme.get_theme();
        let app_state = cx.global::<GlobalAppState>().0.clone();

        let top_bar = top_bar::render(cx, self.sidebar_open, &self.search_bar_state);

        let project_list = project_list::render(cx, self.search_bar_state.read(cx));

        let sidebar = sidebar::render(cx, self.sidebar_open);

        let modal_overlay = div()
            .id("modal_overlay")
            .absolute()
            .size_full()
            .bg(rgba(0x00_00_00_50))
            .opacity(0.0)
            .with_transition("modal_overlay")
            .transition_when(
                !app_state.modal_active,
                Duration::from_millis(10),
                EaseInOutCubic,
                |style| style.opacity(0.0),
            )
            .transition_when(
                app_state.modal_active,
                Duration::from_millis(10),
                EaseInOutCubic,
                |style| style.opacity(1.0),
            )
            .on_click(|_, _, cx: &mut App| {
                let app_state = cx.global::<GlobalAppState>().0.clone();

                if app_state.modal_active {
                    cx.stop_propagation();
                }
            });

        let modal_blocker = div()
            .id("modal_blocker")
            .absolute()
            .size_full()
            .bg(rgba(0x00_00_00_50))
            .occlude()
            .on_click(|_, _, cx: &mut App| {
                cx.stop_propagation();
            });

        let div = div()
            .size_full()
            .bg(theme.background_weak)
            .flex()
            .flex_col()
            .child(top_bar)
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_row()
                    .child(project_list)
                    .child(sidebar),
            )
            .child(modal_overlay)
            .when(app_state.modal_active, |this: Div| {
                this.child(modal_blocker)
            });

        steal_focus!(cx, div)
    }
}

pub fn text_button(
    id: impl Into<ElementId>,
    label: &'static str,
    icon: Option<&'static str>,
    theme: &Theme,
    disabled_if: Option<bool>,
) -> Stateful<Div> {
    let disabled = disabled_if.unwrap_or(false);

    let mut button = div()
        .id(id)
        .h(px(32.0))
        .bg(theme.surface)
        .text_color(theme.text)
        .flex()
        .items_center()
        .px_3()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .when(!disabled, |this: Stateful<Div>| {
            this.cursor_pointer()
                .hover(|style| style.bg(theme.surface_strong))
                .active(|style| style.bg(theme.background_weak))
        });

    match icon {
        Some(icon_str) => {
            button = button
                .w_full()
                .child(div().w(px(24.0)).flex_none().child(icon_str))
                .child(div().flex_1().flex().justify_center().child(label))
                .child(div().w(px(24.0)).flex_none());
        }
        None => {
            button = button.justify_center().child(label);
        }
    }

    button
}
