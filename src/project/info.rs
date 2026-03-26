use std::cmp::Ordering;

use bytesize::ByteSize;
use git2::{BranchType, Commit, Index, Oid, Repository, Revwalk};
use tokei::{LanguageType, Languages};

use crate::project::Project;

#[derive(Debug)]
pub struct ProjectInfo
{
    pub line_count: usize,
    pub language_percentage: Vec<(LanguageType, f64)>,
    pub project_size: ByteSize,
    pub file_count: usize,
    pub branches: Vec<String>,
    pub current_branch: usize,
    pub last_commit: String,
    pub commit_count: usize,
    pub authors: Vec<(String, f64)>,
}

impl Project
{
    pub fn info(&self) -> Option<ProjectInfo>
    {
        let repo: Repository = Repository::open(&self.path).ok()?;

        let index: Index = repo.index().ok()?;

        let head: git2::Reference<'_> = repo.head().ok()?;

        let mut languages: Languages = Languages::new();
        languages.get_statistics(
            &self.template.included_paths(&self.path),
            &self.template.excluded_paths(),
            &tokei::Config::default(),
        );

        let mut line_count: usize = 0;
        for language in languages.values()
        {
            line_count += language.code;
        }

        let mut language_percentage: Vec<(LanguageType, f64)> = vec![];
        for (language_type, language) in languages
        {
            let percentage: f64 = (language.code as f64 / line_count as f64) * 100.0;

            language_percentage.push((language_type, percentage));
        }
        language_percentage.sort_by(|l, p| p.1.partial_cmp(&l.1).unwrap_or(Ordering::Equal));

        let mut project_size: ByteSize = ByteSize::default();
        for entry in index.iter()
        {
            project_size += ByteSize::b(entry.file_size as u64);
        }

        let file_count: usize = index.len();

        let branches: Vec<String> = match repo.branches(Some(BranchType::Local))
        {
            Ok(branches) => branches
                .filter_map(|branch| {
                    let (branch, _) = branch.ok()?;
                    let name = branch.name().ok()??.to_string();
                    Some(name)
                })
                .collect(),
            Err(_) => return None,
        };

        let current_branch: usize = {
            let branch_name = head.shorthand()?;

            branches.iter().position(|b| b == branch_name)?
        };

        let last_commit: String = head.peel_to_commit().ok()?.summary()?.to_string();

        let commit_count: usize = {
            let mut revwalk: Revwalk = repo.revwalk().ok()?;
            revwalk.push_head().ok()?;

            revwalk.count()
        };

        let authors: Vec<(String, f64)> = {
            let mut revwalk: Revwalk = repo.revwalk().ok()?;
            revwalk.push_head().ok()?;

            let mut authors: Vec<(String, usize)> = vec![];
            for oid in revwalk
            {
                let oid: Oid = oid.ok()?;
                let commit: Commit<'_> = repo.find_commit(oid).ok()?;
                let author: String = commit.author().name()?.to_string();

                if let Some(author) = authors.iter_mut().find(|(name, _)| name == &author)
                {
                    author.1 += 1;
                }
                else
                {
                    authors.push((author, 1));
                }
            }

            let mut authors: Vec<(String, f64)> = authors
                .into_iter()
                .map(|(name, commits)| {
                    let percentage = (commits as f64 / commit_count as f64) * 100.0;
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
