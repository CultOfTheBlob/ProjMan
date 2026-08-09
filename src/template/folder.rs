use crate::template::project_context::ProjectContext;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Folder {
    pub name: String,
    pub sub_dirs: Vec<Self>,
}

impl Folder {
    pub fn resolve(&self, root: &Path, ctx: &ProjectContext) -> Vec<PathBuf> {
        let formatted_name = ctx.format(&self.name);
        let current_dir = root.join(formatted_name);

        let mut dirs = vec![current_dir];
        for sub in &self.sub_dirs {
            let current_dir = &dirs[0];

            dirs.extend(sub.resolve(current_dir, ctx));
        }

        dirs
    }
}
