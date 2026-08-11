use crate::{
    app_state::AppState,
    prelude::*,
    project::{Existant, Project},
    template::Template,
};
use bytesize::ByteSize;
use git2::{BranchType, Index, Reference, Repository};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, sync::Arc};
use tokei::{Config as TokeiConfig, Language, LanguageType, Languages};

mod display;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub line_count: usize,
    pub language_percentage: Vec<(LanguageType, f32)>,
    pub project_size: String,
    pub file_count: usize,
    pub branches: Vec<String>,
    pub current_branch: usize,
    pub last_commit: String,
    pub commit_count: usize,
    pub authors: Vec<(String, f32)>,
}

impl Project<Existant> {
    pub fn info(&self, app_state: &Arc<AppState>) -> Result<ProjectInfo> {
        let repo =
            Repository::open(&self.path).map_err(|err| Error::GetProjectInfo(err.to_string()))?;

        let index = repo
            .index()
            .map_err(|err| Error::GetProjectInfo(err.to_string()))?;

        let head = repo
            .head()
            .map_err(|err| Error::GetProjectInfo(err.to_string()))?;

        let template = self.get_template(app_state)?;
        let (line_count, language_percentage) = get_language_stats(self, template);

        let (project_size, file_count) = get_size_and_file_count(&index);
        let branches = get_branches(&repo)?;
        let current_branch = get_current_branch_index(&head, &branches)?;
        let last_commit = get_last_commit_summary(&head)?;
        let (commit_count, authors) = get_commit_history_stats(&repo)?;

        Ok(ProjectInfo {
            line_count,
            language_percentage,
            project_size,
            file_count,
            branches,
            current_branch,
            last_commit,
            commit_count,
            authors,
        })
    }
}

fn get_language_stats(
    project: &Project<Existant>,
    template: &Template,
) -> (usize, Vec<(LanguageType, f32)>) {
    let mut languages = Languages::new();
    let included_paths = template.included_paths(&project.path);

    if !included_paths.is_empty() {
        languages.get_statistics(
            &included_paths,
            &template.excluded_paths(),
            &TokeiConfig::default(),
        );
    }

    let line_count = languages.values().map(|language| language.code).sum();

    let mut percentages = languages
        .into_iter()
        .map(|(language_type, language): (LanguageType, Language)| {
            let percentage = (language.code as f32 / line_count as f32) * 100.0;
            (language_type, percentage)
        })
        .collect::<Vec<(LanguageType, f32)>>();

    percentages.sort_by(|a: &(LanguageType, f32), b: &(LanguageType, f32)| {
        b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
    });

    (line_count, percentages)
}

fn get_size_and_file_count(index: &Index) -> (String, usize) {
    let total_bytes = index.iter().map(|entry| u64::from(entry.file_size)).sum();
    let project_size = ByteSize::b(total_bytes).display().iec().to_string();

    let file_count = index.len();

    (project_size, file_count)
}

fn get_branches(repo: &Repository) -> Result<Vec<String>> {
    let branches = repo
        .branches(Some(BranchType::Local))
        .map_err(|err| Error::GetProjectInfo(err.to_string()))?
        .filter_map(|branch| {
            let (branch, _) = branch.ok()?;
            let name = branch.name().ok()??.to_owned();
            Some(name)
        })
        .collect();

    Ok(branches)
}

fn get_current_branch_index(head: &Reference, branches: &[String]) -> Result<usize> {
    let current_branch_name = head
        .shorthand()
        .ok_or(Error::GetProjectInfo(String::new()))?;

    branches
        .iter()
        .position(|branch: &String| branch == current_branch_name)
        .ok_or(Error::GetProjectInfo(String::new()))
}

fn get_last_commit_summary(head: &Reference) -> Result<String> {
    let summary = head
        .peel_to_commit()
        .map_err(|err| Error::GetProjectInfo(err.to_string()))?
        .summary()
        .ok_or(Error::GetProjectInfo(String::new()))?
        .to_owned();

    Ok(summary)
}

fn get_commit_history_stats(repo: &Repository) -> Result<(usize, Vec<(String, f32)>)> {
    let mut revwalk = repo
        .revwalk()
        .map_err(|err| Error::GetProjectInfo(err.to_string()))?;

    revwalk
        .push_head()
        .map_err(|err| Error::GetProjectInfo(err.to_string()))?;

    let mut commit_count = 0;
    let mut author_counts = HashMap::new();

    for oid in revwalk {
        let oid = oid.map_err(|err| Error::GetProjectInfo(err.to_string()))?;
        let commit = repo
            .find_commit(oid)
            .map_err(|err| Error::GetProjectInfo(err.to_string()))?;

        let author = commit
            .author()
            .name()
            .ok_or(Error::GetProjectInfo(String::new()))?
            .to_owned();

        *author_counts.entry(author).or_insert(0) += 1;
        commit_count += 1;
    }

    let mut authors = author_counts
        .into_iter()
        .map(|(name, count)| {
            let percentage = (count as f32 / commit_count as f32) * 100.0;
            (name, percentage)
        })
        .collect::<Vec<(String, f32)>>();

    authors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
    authors.truncate(4);

    Ok((commit_count, authors))
}
