use crate::project::info::ProjectInfo;
use colored::Colorize as _;
use std::fmt::{Display, Formatter, Result};

impl Display for ProjectInfo {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        const TOP_LEFT: &str = "┌";
        const BOTTOM_LEFT: &str = "└";
        const LINE: &str = "│";

        let section = |f: &mut Formatter<'_>, title: &str, content: &str| -> Result {
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

            for line in content.lines() {
                let visible = {
                    let mut len = 0;
                    let mut in_escape = false;
                    for char in line.chars() {
                        if char == '\x1b' {
                            in_escape = true;
                        } else if in_escape {
                            if char == 'm' {
                                in_escape = false;
                            }
                        } else {
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

            for (i, branch) in self.branches.iter().enumerate() {
                if i == self.current_branch {
                    let branch =
                        format!("{}{} {}\n", LINE.dimmed(), "●".green(), branch.bold());

                    branches.push_str(&branch);
                } else {
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

            for (language, percentage) in &self.language_percentage {
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

            for (author, percentage) in &self.authors {
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
                last_commit,
                LINE.dimmed(),
                commit_count
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
                line_count,
                LINE.dimmed(),
                file_count,
                LINE.dimmed(),
                project_size
            )
        };
        section(formatter, "Metadata", &metadata)?;

        Ok(())
    }
}
