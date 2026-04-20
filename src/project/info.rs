use std::{
    cmp::{self, Ordering},
    fmt::{Display, Formatter, Result as FmtResult},
};

use bytesize::ByteSize;
use color_eyre::owo_colors::OwoColorize as _;
use git2::{BranchType, Repository, Revwalk};
use serde::{Deserialize, Serialize};
use tokei::{Config as TokeiConfig, LanguageType, Languages};

use crate::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo
{
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

impl Project
{
    pub fn info(&self) -> Option<ProjectInfo>
    {
        let repo = Repository::open(&self.path).ok()?;

        let index = repo.index().ok()?;

        let head = repo.head().ok()?;

        let mut languages = Languages::new();

        if let paths = self.template.included_paths(&self.path)
            && !paths.is_empty()
        {
            languages.get_statistics(
                &paths,
                &self.template.excluded_paths(),
                &TokeiConfig::default(),
            );
        }

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
        let project_size = project_size.display().iec().to_string();

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

            authors[0..cmp::min(authors.len(), 4)].to_vec()
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

impl Display for ProjectInfo
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult
    {
        const TOP_LEFT: &str = "┌";
        const BOTTOM_LEFT: &str = "└";
        const LINE: &str = "│";

        let section = |f: &mut Formatter<'_>, title: &str, content: &str| -> FmtResult {
            const LEFT_FIXED: usize = 9;
            const LENGTH: usize = 80;

            let title_visible = title.len() + 2;
            let top_dashes = "─".repeat(8);
            let title_section = format!("<{title}>");
            let after_title = LENGTH.saturating_sub(LEFT_FIXED - 1 + title_visible);
            let top_right = "─".repeat(after_title);
            let bottom_line = "─".repeat(LENGTH);

            writeln!(
                f,
                "{}{}{}{}{}",
                TOP_LEFT.dimmed(),
                top_dashes.dimmed(),
                title_section.bold().cyan(),
                top_right.dimmed(),
                "┐".dimmed(),
            )?;

            for line in content.lines()
            {
                let visible = {
                    let mut len = 0;
                    let mut in_escape = false;
                    for char in line.chars()
                    {
                        if char == '\x1b'
                        {
                            in_escape = true;
                        }
                        else if in_escape
                        {
                            if char == 'm'
                            {
                                in_escape = false;
                            }
                        }
                        else
                        {
                            len += 1;
                        }
                    }
                    len - 1
                };

                let pad = LENGTH.saturating_sub(visible);
                writeln!(f, "{}{}{}", line, " ".repeat(pad), LINE.dimmed())?;
            }

            writeln!(
                f,
                "{}{}{}",
                BOTTOM_LEFT.dimmed(),
                bottom_line.dimmed(),
                "┘".dimmed(),
            )?;

            writeln!(f)
        };

        let branches = {
            let mut branches = String::new();

            for (i, branch) in self.branches.iter().enumerate()
            {
                if i == self.current_branch
                {
                    let branch = format!("{}{} {}\n", LINE.dimmed(), "●".green(), branch.bold());

                    branches.push_str(&branch);
                }
                else
                {
                    let branch = format!("{} {}\n", "  ○".dimmed(), branch.dimmed());

                    branches.push_str(&branch);
                }
            }

            branches
        };
        section(formatter, "Branches", &branches)?;

        let languages = {
            let mut languages = String::new();

            let max_label_len = self
                .language_percentage
                .iter()
                .map(|(l, p)| l.name().len() + format!("({p:.1}%)").len())
                .max()
                .unwrap_or(0);

            for (language, percentage) in &self.language_percentage
            {
                #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let filled = ((percentage / 100.0) * 20.0) as usize;

                let label_len = language.name().len() + format!("{percentage:.1}%").len();

                let language = format!(
                    "{}{} {} {}{}  {}{}\n",
                    LINE.dimmed(),
                    "●".green(),
                    format!("{language:?}").bold(),
                    format!("({percentage:.1}%)").dimmed(),
                    " ".repeat(max_label_len - label_len),
                    "█".repeat(filled).green(),
                    "░".repeat(20 - filled).dimmed()
                );

                languages.push_str(&language);
            }

            languages
        };
        section(formatter, "Languages", &languages)?;

        let authors = {
            let mut authors = String::new();

            for (author, percentage) in &self.authors
            {
                let author = format!(
                    "{}{} {} {}\n",
                    LINE.dimmed(),
                    "●".green(),
                    author.bold(),
                    format!("({percentage:.1}%)").dimmed()
                );

                authors.push_str(&author);
            }

            authors
        };
        section(formatter, "Authors", &authors)?;

        let commits = {
            let last_commit = format!(
                "{} {:.60}",
                "Last Commit:      ".dimmed(),
                self.last_commit.bold()
            );
            let commit_count = format!(
                "{} {}",
                "Number of Commits:".dimmed(),
                self.commit_count.to_string().bold().yellow()
            );

            format!(
                "{}{}\n{}{}\n",
                LINE.dimmed(),
                &last_commit,
                LINE.dimmed(),
                &commit_count
            )
        };
        section(formatter, "Commits", &commits)?;

        let metadata = {
            let line_count = format!(
                "{} {}",
                "Lines of Code:".dimmed(),
                self.line_count.to_string().bold().yellow()
            );
            let file_count = format!(
                "{} {}",
                "Files:        ".dimmed(),
                self.file_count.to_string().bold().yellow()
            );
            let project_size = format!(
                "{} {}",
                "Size:         ".dimmed(),
                self.project_size.clone().bold().yellow()
            );

            format!(
                "{}{}\n{}{}\n{}{}\n",
                LINE.dimmed(),
                &line_count,
                LINE.dimmed(),
                &file_count,
                LINE.dimmed(),
                &project_size
            )
        };
        section(formatter, "Metadata", &metadata)?;

        Ok(())
    }
}
