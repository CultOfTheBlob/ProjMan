use crate::template::{command::Command, file::File, folder::Folder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TemplateConfig {
    pub dir_structure: Vec<Folder>,
    pub files: Vec<File>,
    pub build: Vec<Command>,
    pub run: Vec<Command>,
    pub included_paths: Vec<String>,
    pub excluded_paths: Vec<String>,
}
