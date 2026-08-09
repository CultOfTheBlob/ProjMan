use serde::{Deserialize, Serialize};
use std::fmt::{Display, Result as FmtResult};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.program, self.args.join(" "))
    }
}
