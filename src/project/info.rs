use std::cmp::Ordering;

use bytesize::ByteSize;
use git2::{BranchType, Repository, Revwalk};
use tokei::{Config as TokeiConfig, LanguageType, Languages};

use crate::project::Project;

#[derive(Debug)]
pub struct ProjectInfo
{
    pub line_count: usize,
    pub language_percentage: Vec<(LanguageType, f32)>,
    pub project_size: ByteSize,
    pub file_count: usize,
    pub branches: Vec<String>,
    pub current_branch: usize,
    pub last_commit: String,
    pub commit_count: usize,
    pub authors: Vec<(String, f32)>,
}

impl Project
{
    pub fn info(&self) -> Option<ProjectInfo>
    {
        let repo = Repository::open(&self.path).ok()?;

        let index = repo.index().ok()?;

        let head = repo.head().ok()?;

        let mut languages = Languages::new();
        languages.get_statistics(
            &self.template.included_paths(&self.path),
            &self.template.excluded_paths(),
            &TokeiConfig::default(),
        );

        let mut line_count = 0;
        for language in languages.values()
        {
            line_count += language.code;
        }

        let mut language_percentage: Vec<(LanguageType, f32)> = vec![];
        for (language_type, language) in languages
        {
            let percentage: f32 = (language.code as f32 / line_count as f32) * 100.0;

            language_percentage.push((language_type, percentage));
        }
        language_percentage.sort_by(|l, p| p.1.partial_cmp(&l.1).unwrap_or(Ordering::Equal));

        let mut project_size = ByteSize::default();
        for entry in index.iter()
        {
            project_size += ByteSize::b(entry.file_size as u64);
        }

        let file_count = index.len();

        let branches: Vec<String> = match repo.branches(Some(BranchType::Local))
        {
            Ok(branches) => branches
                .filter_map(|branch| {
                    let (branch, _) = branch.ok()?;
                    let name = branch.name().ok()??.to_owned();
                    Some(name)
                })
                .collect(),
            Err(_) => return None,
        };

        let current_branch = {
            let branch_name = head.shorthand()?;

            branches.iter().position(|b| b == branch_name)?
        };

        let last_commit = head.peel_to_commit().ok()?.summary()?.to_owned();

        let commit_count = {
            let mut revwalk: Revwalk = repo.revwalk().ok()?;
            revwalk.push_head().ok()?;

            revwalk.count()
        };

        let authors = {
            let mut revwalk: Revwalk = repo.revwalk().ok()?;
            revwalk.push_head().ok()?;

            let mut authors: Vec<(String, usize)> = vec![];
            for oid in revwalk
            {
                let oid = oid.ok()?;
                let commit = repo.find_commit(oid).ok()?;
                let author = commit.author().name()?.to_owned();

                if let Some(author) = authors.iter_mut().find(|(name, _)| name == &author)
                {
                    author.1 += 1;
                }
                else
                {
                    authors.push((author, 1));
                }
            }

            let mut authors: Vec<(String, f32)> = authors
                .into_iter()
                .map(|(name, commits)| {
                    let percentage = (commits as f32 / commit_count as f32) * 100.0;
                    (name, percentage)
                })
                .collect();

            authors.sort_by(|a, p| p.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

            if authors.len() > 4
            {
                authors[0..4].to_vec()
            }
            else
            {
                authors
            }
        };

        Some(ProjectInfo {
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
