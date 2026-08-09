use std::path::Path;

use crate::template::project_context::ProjectContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct File {
    pub path: String,
    pub contents: String,
}

impl File {
    pub fn resolve(&self, root: &Path, ctx: &ProjectContext) -> Self {
        Self {
            path: root
                .join(ctx.format(&self.path))
                .to_string_lossy()
                .to_string(),
            contents: ctx.format(&self.contents),
        }
    }
}
