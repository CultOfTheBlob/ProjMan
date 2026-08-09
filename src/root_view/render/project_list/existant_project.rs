use crate::{
    app_state::{AppState, GlobalAppState},
    config::Config,
    log::Log,
    project::{Existant, Project},
    root_view::RootView,
};
use gpui::*;
use image::{Frame, RgbaImage};
use resvg::{
    tiny_skia,
    usvg::{self, Tree},
};
use std::fs;
use std::{path::Path, sync::Arc};

pub fn render(
    cx: &Context<RootView>,
    project: &Arc<Project<Existant>>,
    icon: &Path,
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
        let project = project.clone();

        move |event: &ClickEvent, _: &mut Window, cx: &mut App| {
            if event.click_count() == 2 {
                if let Err(err) = app_state.run_project(&project) {
                    Log::Error.log(&err.to_string());

                    return;
                }

                root_view.update(cx, set_selected_project_index(None));
            } else {
                root_view.update(cx, set_selected_project_index(Some(index)));
            }

            cx.stop_propagation();
        }
    };

    let icon = load_svg(icon, 128).map_or_else(
        || div().size_16().into_any_element(),
        |render_img| img(render_img).size_16().flex_shrink_0().into_any_element(),
    );

    let project_name = div().text_color(theme.accent).child(project.name.clone());

    let template_name = div()
        .text_color(theme.text_muted)
        .child(project.template_name.clone());

    let path = div()
        .text_color(theme.text_muted)
        .child(project.path.to_string_lossy().into_owned());

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
                .flex_col()
                .w_full()
                .gap_2()
                .child(project_name)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_4()
                        .text_color(theme.text_muted)
                        .child(template_name)
                        .child(path),
                ),
        )
}

pub fn load_svg(path: &Path, target_pixels: u32) -> Option<Arc<RenderImage>> {
    let svg_data = fs::read(path).ok()?;
    let opt = usvg::Options::default();
    let tree = Tree::from_data(&svg_data, &opt).ok()?;

    let mut pixmap = tiny_skia::Pixmap::new(target_pixels, target_pixels)?;

    let bbox = tree.size();
    let scale_x = target_pixels as f32 / bbox.width();
    let scale_y = target_pixels as f32 / bbox.height();
    let scale = scale_x.min(scale_y);

    let transform = tiny_skia::Transform::from_scale(scale, scale);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let mut bgra_bytes = Vec::with_capacity(pixmap.data().len());
    for pixel in pixmap.pixels() {
        let c = pixel.demultiply();

        bgra_bytes.push(c.blue());
        bgra_bytes.push(c.green());
        bgra_bytes.push(c.red());
        bgra_bytes.push(c.alpha());
    }

    let bgra_image = RgbaImage::from_raw(target_pixels, target_pixels, bgra_bytes)?;
    let frame = Frame::new(bgra_image);

    Some(Arc::new(RenderImage::new(vec![frame])))
}
