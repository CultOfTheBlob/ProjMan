use crate::project::Project;

pub struct ProjectContext<'a> {
    pub name: &'a str,
    pub repo: &'a str,
    pub license: &'a str,
}

impl ProjectContext<'_> {
    pub fn format(&self, target: &str) -> String {
        target
            .replace("#{name}", self.name)
            .replace("#{repo}", self.repo)
            .replace("#{license}", self.license)
    }
}

impl<'a, State> From<&'a Project<State>> for ProjectContext<'a> {
    fn from(value: &'a Project<State>) -> Self {
        Self {
            name: &value.name,
            repo: &value.repo,
            license: &value.license,
        }
    }
}
