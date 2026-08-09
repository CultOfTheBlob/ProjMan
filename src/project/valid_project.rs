use crate::{
    log::Log,
    project::{Existant, Nonexistant, Project, Unvalidated},
};
use serde::ser::{Serialize, Serializer};
use std::{marker::PhantomData, sync::Arc};

#[derive(Debug, Clone)]
pub enum ValidProject {
    Existant(Arc<Project<Existant>>),
    Nonexistant(Arc<Project<Nonexistant>>),
}

impl Project<Unvalidated> {
    pub(super) fn validate(self) -> ValidProject {
        let project_exists = self.exists().unwrap_or_else(|err| {
            Log::Error.log(&err.to_string());

            false
        });

        if project_exists {
            ValidProject::Existant(Arc::new(Project {
                name: self.name,
                path: self.path,
                repo: self.repo,
                license: self.license,
                template_name: self.template_name,

                state: PhantomData,
            }))
        } else {
            ValidProject::Nonexistant(Arc::new(Project {
                name: self.name,
                path: self.path,
                repo: self.repo,
                license: self.license,
                template_name: self.template_name,

                state: PhantomData,
            }))
        }
    }
}

impl Serialize for ValidProject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Existant(p) => p.as_ref().serialize(serializer),
            Self::Nonexistant(p) => p.as_ref().serialize(serializer),
        }
    }
}
