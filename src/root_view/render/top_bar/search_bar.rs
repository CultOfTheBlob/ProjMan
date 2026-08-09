use gpui::*;
use gpui_component::input::{Input, InputState};

use crate::root_view::render::input;

pub fn render(input_state: &Entity<InputState>) -> Div {
    div().child(input!(Input::new(input_state).w(px(800.0))))
}
