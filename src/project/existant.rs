use crate::{
    app_state::AppState,
    prelude::*,
    project::{Existant, Project},
};

impl Project<Existant> {
    pub fn run(&self, app_state: &AppState) -> Result<()> {
        app_state.get_template(&self.template_name)?.run(&self.path)
    }
}
